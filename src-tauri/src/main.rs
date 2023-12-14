// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use error::Result;
use features::hotkey::{HotKeyManager, HotkeyState};
use log::{warn, LevelFilter};
use tauri::{
    async_runtime::Mutex, AppHandle, CustomMenuItem, GlobalWindowEvent, Manager, SystemTray,
    SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_log::LogTarget;

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

    #[cfg(debug_assertions)]
    const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Debug;
    #[cfg(debug_assertions)]
    const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::Webview];
    #[cfg(not(debug_assertions))]
    const LOG_TARGETS: [LogTarget; 1] = [LogTarget::LogDir];
    #[cfg(not(debug_assertions))]
    const LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Info;

    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets(LOG_TARGETS)
                .level(LOG_LEVEL_FILTER)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::fetch_mic_mute_hotkey,
            commands::set_mic_mute_hotkey,
            commands::toggle_mic,
            commands::set_mic_mute_audio_volume,
            commands::get_mic_mute_audio_volume,
            commands::fetch_start_minimized_state,
            commands::set_start_minimized_state,
        ])
        .system_tray(system_tray())
        .on_system_tray_event(|app_handle, event| handle_tray_events(app_handle, event).unwrap())
        .on_window_event(|event| handle_window_events(event).unwrap())
        .manage(hotkey_state)
        .setup(move |app| {
            db::init_db(app.path_resolver().resource_dir());
            features::hotkey::init(hotkey_manager);
            hide_on_startup(&app.app_handle()).unwrap();
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

fn handle_tray_events(app_handle: &AppHandle, event: SystemTrayEvent) -> Result<()> {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => show_window(true, app_handle)?,
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            QUIT_TRAY_ID => app_handle.exit(0),
            VISABILITY_TRAY_ID => show_window(
                !app_handle
                    .get_window("main")
                    .expect("window should be labeled main")
                    .is_visible()?,
                app_handle,
            )?,
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

fn show_window(show: bool, app_handle: &AppHandle) -> Result<()> {
    let window = app_handle.get_window("main").unwrap();
    match show {
        true => {
            window.show()?;
            if window.is_minimized()? {
                window.unminimize()?;
            }
        }
        false => window.hide()?,
    }
    app_handle
        .tray_handle()
        .get_item(VISABILITY_TRAY_ID)
        .set_title(if show { "Hide" } else { "Show" })?;
    Ok(())
}

fn handle_window_events(event: GlobalWindowEvent) -> Result<()> {
    match event.event() {
        WindowEvent::Resized(_) => {
            if event
                .window()
                .app_handle()
                .get_window("main")
                .expect("window should be labeled main")
                .is_minimized()?
            {
                show_window(false, &event.window().app_handle())?
            }
        }
        WindowEvent::CloseRequested { api, .. } => {
            show_window(false, &event.window().app_handle())?;
            api.prevent_close()
        }
        _ => {}
    }
    Ok(())
}

fn hide_on_startup(app_handle: &AppHandle) -> Result<()> {
    match db::fetch_start_minimized_state() {
        Ok(start_minimized_state) => show_window(!start_minimized_state, app_handle)?,
        Err(error) => warn!("Failed to fetch start_minimized_state {error}"),
    }
    Ok(())
}
