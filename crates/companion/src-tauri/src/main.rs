#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use std::process::Command;
use std::env;

const PROXY_SERVER: &str = "127.0.0.1:3128";

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
            .args(["-addstore", "-f", "ROOT", cert_path_str])
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
            .args([
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
        let cert_dest = "/usr/local/share/ca-certificates/sentiric-ca.crt";
        
        let copy_output = Command::new("sudo")
            .args(["cp", cert_path_str, cert_dest])
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
    println!("Sistem proxy'si etkinleştiriliyor: {}", PROXY_SERVER);

    #[cfg(target_os = "windows")]
    {
        // Windows: Internet Settings registry anahtarlarını güncelle.
        Command::new("reg")
            .args(["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "1", "/f"])
            .output().map_err(|e| format!("ProxyEnable ayarı yapılamadı: {}", e))?;
        Command::new("reg")
            .args(["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", "/v", "ProxyServer", "/d", PROXY_SERVER, "/f"])
            .output().map_err(|e| format!("ProxyServer ayarı yapılamadı: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: Aktif ağ servisini bul ve `networksetup` ile proxy'yi ayarla.
        let services = ["Wi-Fi", "Ethernet", "Display Ethernet"]; 
        for service in services.iter() {
            Command::new("networksetup").args(["-setwebproxy", service, "127.0.0.1", "3128"]).output().ok();
            Command::new("networksetup").args(["-setsecurewebproxy", service, "127.0.0.1", "3128"]).output().ok();
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux (GNOME/GTK): `gsettings` kullanarak proxy ayarlarını yap.
        Command::new("gsettings").args(["set", "org.gnome.system.proxy", "mode", "'manual'"]).output().map_err(|e| format!("gsettings mode ayarlanamadı: {}", e))?;
        Command::new("gsettings").args(["set", "org.gnome.system.proxy.http", "host", "'127.0.0.1'"]).output().map_err(|e| format!("gsettings http host ayarlanamadı: {}", e))?;
        Command::new("gsettings").args(["set", "org.gnome.system.proxy.http", "port", "3128"]).output().map_err(|e| format!("gsettings http port ayarlanamadı: {}", e))?;
        Command::new("gsettings").args(["set", "org.gnome.system.proxy.https", "host", "'127.0.0.1'"]).output().map_err(|e| format!("gsettings https host ayarlanamadı: {}", e))?;
        Command::new("gsettings").args(["set", "org.gnome.system.proxy.https", "port", "3128"]).output().map_err(|e| format!("gsettings https port ayarlanamadı: {}", e))?;
    }
    
    println!("Sistem proxy'si başarıyla etkinleştirildi.");
    Ok(())
}

#[tauri::command]
fn disable_system_proxy() -> Result<(), String> {
    println!("Sistem proxy'si devre dışı bırakılıyor.");

    #[cfg(target_os = "windows")]
    {
        Command::new("reg")
            .args(["add", "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Internet Settings", "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "0", "/f"])
            .output().map_err(|e| format!("Proxy devre dışı bırakılamadı: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        let services = ["Wi-Fi", "Ethernet", "Display Ethernet"];
        for service in services.iter() {
            Command::new("networksetup").args(["-setwebproxystate", service, "off"]).output().ok();
            Command::new("networksetup").args(["-setsecurewebproxystate", service, "off"]).output().ok();
        }
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("gsettings").args(["set", "org.gnome.system.proxy", "mode", "'none'"]).output().map_err(|e| format!("gsettings mode sıfırlanamadı: {}", e))?;
    }

    println!("Sistem proxy'si başarıyla devre dışı bırakıldı.");
    Ok(())
}


fn main() {
    // ... (main fonksiyonunun geri kalanı aynı)
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