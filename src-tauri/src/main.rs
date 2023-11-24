// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use features::hotkey::{HotKeyManager, HotkeyState};
use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_autostart::MacosLauncher;

mod commands;
mod db;
mod error;
mod features;

const VISABILITY_TRAY_ID: &str = "visability";
const QUIT_TRAY_ID: &str = "quit";
pub const APP_INFO: app_dirs2::AppInfo = app_dirs2::AppInfo {
    name: "JUtils",
    author: "com.github.JonasSeifried.JUtils",
};

fn main() {
    let hotkey_state = HotkeyState(Arc::new(Mutex::new(HotKeyManager::new())));
    let hotkey_manager = Arc::clone(&hotkey_state.0);

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            commands::fetch_mic_mute_hotkey,
            commands::set_mic_mute_hotkey,
            commands::toggle_mic,
            commands::set_mic_mute_audio_volume,
            commands::get_mic_mute_audio_volume
        ])
        .system_tray(system_tray())
        .on_system_tray_event(handle_tray_events)
        .manage(hotkey_state)
        .setup(move |_app| {
            db::init_db();
            features::hotkey::init(hotkey_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn system_tray() -> SystemTray {
    let quit = CustomMenuItem::new(QUIT_TRAY_ID.to_string(), "Quit");
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
            QUIT_TRAY_ID => app_handle.exit(0),
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
