// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod error;
mod features;
mod hotkey;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::fetch_mic_mute_hotkey,
            commands::set_mic_mute_hotkey,
            commands::toggle_mic,
            commands::set_auto_launch,
            commands::get_auto_launch
        ])
        .setup(|app| {
            db::init_db(&app.package_info().name);
            features::auto_launch::init().expect("Could not init auto_launch");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
