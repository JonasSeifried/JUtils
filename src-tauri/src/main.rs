// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const VISABILITY_TRAY_ID: &str = "visability";

use std::sync::Arc;

use features::hotkey::{HotKeyManager, HotkeyState};
use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

mod commands;
mod db;
mod error;
mod features;

fn main() {
    let hotkey_state = HotkeyState(Arc::new(Mutex::new(HotKeyManager::new())));
    let hotkey_manager = Arc::clone(&hotkey_state.0);
    features::hotkey::init(hotkey_manager);

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
        .system_tray(system_tray())
        .on_system_tray_event(handle_tray_events)
        .manage(hotkey_state)
        .setup(move |app| {
            db::init_db(&app.package_info().name);
            features::auto_launch::init().expect("Could not init auto_launch");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new(VISABILITY_TRAY_ID.to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn handle_tray_events(app_handle: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => show_window(true, app_handle),
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "quit" => app_handle.exit(0),
            VISABILITY_TRAY_ID => show_window(
                !app_handle.get_window("main").unwrap().is_visible().unwrap(),
                app_handle,
            ),
            _ => {}
        },
        _ => {}
    }
}

fn show_window(show: bool, app_handle: &AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    match show {
        true => window.show().unwrap(),
        false => window.hide().unwrap(),
    }
    app_handle
        .tray_handle()
        .get_item(VISABILITY_TRAY_ID)
        .set_title(if show { "Hide" } else { "Show" })
        .unwrap();
}
