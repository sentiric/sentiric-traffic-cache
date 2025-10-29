#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

// --- YENİ KOMUTLAR ---

#[tauri::command]
fn install_ca_certificate() -> Result<(), String> {
    // TODO: İşletim sistemine özel sertifika yükleme mantığı buraya gelecek.
    // Örnek: Windows için certutil, macOS için `security` komutu, Linux için `trust` komutu.
    println!("Sertifika yükleme komutu çağrıldı.");
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
        // --- YENİ INVOKE HANDLER ---
        // Frontend'in Rust fonksiyonlarını çağırabilmesi için komutları kaydediyoruz.
        .invoke_handler(tauri::generate_handler![
            install_ca_certificate,
            enable_system_proxy,
            disable_system_proxy
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}