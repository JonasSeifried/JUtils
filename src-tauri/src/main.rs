// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use features::hotkey::HotKeyManager;
use tauri::{Manager, SystemTray};

mod commands;
mod db;
mod error;
mod features;

fn main() {
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

fn system_tray() -> SystemTray {
    let tray = SystemTray::new();

    tray
}
