// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use features::hotkey::HotKeyManager;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

mod commands;
mod db;
mod error;
mod features;

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::fetch_mic_mute_hotkey,
            commands::set_mic_mute_hotkey,
            commands::toggle_mic,
            commands::set_auto_launch,
            commands::get_auto_launch,
            commands::set_mic_mute_audio_volume,
            commands::get_mic_mute_audio_volume
        ])
        .system_tray(tray.with_menu(tray_menu))
        .on_system_tray_event(handle_tray_events)
        .manage(HotKeyManager::new())
        .setup(move |app| {
            db::init_db(&app.package_info().name);
            features::auto_launch::init().expect("Could not init auto_launch");
            features::hotkey::init(app.handle().state::<HotKeyManager>().inner(), app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn system_tray() -> SystemTray {}

fn handle_tray_events(app_handle: &AppHandle, event: SystemTrayEvent) {
    println!("test");
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app_handle.exit(0),
            "hide" => {
                let window = app_handle.get_window("main").unwrap();
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
