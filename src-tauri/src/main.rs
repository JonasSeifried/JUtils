// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod db;
mod error;
mod hotkey;

fn main() {
    db::init_db();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::fetch_mic_mute_hotkey,
            commands::set_mic_mute_hotkey
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
