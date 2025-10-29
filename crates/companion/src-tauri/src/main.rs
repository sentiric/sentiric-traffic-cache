#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use std::process::Command; // Stdio kaldırıldı
use std::env;

// --- GÜNCELLENMİŞ KOMUT ---

#[tauri::command]
fn install_ca_certificate(app_handle: tauri::AppHandle) -> Result<(), String> {
    // Uygulamanın config dizininden `.certs/ca.crt` dosyasının yolunu bul.
    let cert_path = app_handle.path_resolver()
        .app_config_dir()
        .ok_or("Uygulama yapılandırma dizini bulunamadı.")?
        .join(".certs/ca.crt");

    if !cert_path.exists() {
        return Err(format!("Sertifika dosyası bulunamadı: {:?}", cert_path));
    }

    let cert_path_str = cert_path.to_str().ok_or("Sertifika yolu geçersiz karakterler içeriyor.")?;
    println!("Sertifika yükleniyor: {}", cert_path_str);

    #[cfg(target_os = "windows")]
    {
        // Windows: `certutil` kullanarak sertifikayı "Trusted Root Certification Authorities" deposuna ekle.
        let output = Command::new("certutil")
            .args(["-addstore", "-f", "ROOT", cert_path_str]) // & kaldırıldı (isteğe bağlı, ama clippy önerir)
            .output()
            .map_err(|e| format!("certutil komutu çalıştırılamadı: {}", e))?;
        
        if !output.status.success() {
            return Err(format!("certutil başarısız oldu: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: `security` komutu kullanarak sertifikayı System.keychain'e ekle ve güvenilir yap.
        let output = Command::new("sudo")
            .args([ // & kaldırıldı (isteğe bağlı, ama clippy önerir)
                "security",
                "add-trusted-cert",
                "-d",
                "-r", "trustRoot",
                "-k", "/Library/Keychains/System.keychain",
                cert_path_str,
            ])
            .output()
            .map_err(|e| format!("security komutu çalıştırılamadı: {}", e))?;

        if !output.status.success() {
            return Err(format!("security başarısız oldu: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Sertifikayı `ca-certificates` dizinine kopyala ve `update-ca-certificates` komutunu çalıştır.
        // Bu, Debian/Ubuntu tabanlı sistemlerde çalışır. Diğer dağıtımlar farklı yollar kullanabilir.
        let cert_dest = "/usr/local/share/ca-certificates/sentiric-ca.crt";
        
        let copy_output = Command::new("sudo")
            .args(["cp", cert_path_str, cert_dest]) // & kaldırıldı (CLİPPY HATASININ ÇÖZÜMÜ)
            .output()
            .map_err(|e| format!("cp komutu çalıştırılamadı: {}", e))?;
        if !copy_output.status.success() {
            return Err(format!("Sertifika kopyalanamadı: {}", String::from_utf8_lossy(&copy_output.stderr)));
        }

        let update_output = Command::new("sudo")
            .arg("update-ca-certificates")
            .output()
            .map_err(|e| format!("update-ca-certificates komutu çalıştırılamadı: {}", e))?;
        if !update_output.status.success() {
            return Err(format!("update-ca-certificates başarısız oldu: {}", String::from_utf8_lossy(&update_output.stderr)));
        }
    }
    
    println!("Sertifika başarıyla yüklendi.");
    Ok(())
}


#[tauri::command]
fn enable_system_proxy() -> Result<(), String> {
    // TODO: İşletim sistemine özel proxy etkinleştirme mantığı buraya gelecek.
    // Windows: Registry ayarları, macOS/Linux: `networksetup` / `gsettings`.
    println!("Sistem proxy'sini etkinleştirme komutu çağrıldı.");
    Ok(())
}

#[tauri::command]
fn disable_system_proxy() -> Result<(), String> {
    // TODO: İşletim sistemine özel proxy devre dışı bırakma mantığı buraya gelecek.
    println!("Sistem proxy'sini devre dışı bırakma komutu çağrıldı.");
    Ok(())
}


fn main() {
    // Arka planda ana servis katmanımızı başlat
    std::thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            if let Err(e) = sentiric_service::run().await {
                eprintln!("Core service failed: {}", e);
            }
        });
    });

    let quit = CustomMenuItem::new("quit".to_string(), "Çıkış");
    let show = CustomMenuItem::new("show".to_string(), "Paneli Göster");
    let tray_menu = SystemTrayMenu::new().add_item(show).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            install_ca_certificate,
            enable_system_proxy,
            disable_system_proxy
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}