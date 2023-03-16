#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;
use std::process::Command;
use tauri::{ActivationPolicy, CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

fn main() {
    let mut app = tauri::Builder::default()
        .system_tray(get_system_tray())
        .on_system_tray_event(handle_tray_click)
        .invoke_handler(tauri::generate_handler![])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");
    app.set_activation_policy(ActivationPolicy::Accessory);
    app.run(move |_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

fn get_system_tray() -> SystemTray {
    let title = CustomMenuItem::new("title", "WallBar").disabled();

    let open_wall = CustomMenuItem::new("open_wall".to_string(), "Open Wallpaper");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(title)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(open_wall)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu)
}

fn handle_tray_click(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    let binding = app
        .path_resolver()
        .resolve_resource("open_wall_for_random.scpt")
        .unwrap();
    let open_wall_for_random = binding.to_str().unwrap();
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "open_wall" => {
                Command::new("osascript")
                    .arg(open_wall_for_random)
                    .spawn()
                    .expect("Failed to execute applescript script");
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        },
        _ => {}
    }
}
