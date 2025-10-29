#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}