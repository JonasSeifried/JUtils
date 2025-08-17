use tauri::{
    menu::{Menu, MenuBuilder},
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent, Wry,
};
use tauri_plugin_store::StoreExt;

use crate::error::{Error::UnexpectedError, Result};

const QUIT_I_ID: &str = "quit";
const VISABLITY_I_ID: &str = "visibility";
const TRAY_ICON_ID: &str = "tray_icon";

fn create_tray_menu(app_handle: AppHandle) -> Result<Menu<Wry>> {
    let is_visible = app_handle
        .get_webview_window("main")
        .expect("window should be labeled main")
        .is_visible()?;

    let visiblity_text = if !is_visible {
        "Open JUtils"
    } else {
        "Hide JUtils"
    };

    let menu = MenuBuilder::new(&app_handle)
        .id(TRAY_ICON_ID)
        .text(VISABLITY_I_ID, visiblity_text)
        .separator()
        .text(QUIT_I_ID, "Quit")
        .build()?;

    Ok(menu)
}

pub fn create_system_tray(app_handle: AppHandle) -> Result<()> {
    let menu = create_tray_menu(app_handle.clone())?;

    TrayIconBuilder::with_id(TRAY_ICON_ID)
        .menu(&menu)
        .icon(
            app_handle
                .default_window_icon()
                .ok_or(UnexpectedError(
                    "Failed to set system tray icon".to_string(),
                ))?
                .clone(),
        )
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            QUIT_I_ID => {
                app.exit(0);
            }
            VISABLITY_I_ID => show_window(
                !app.get_webview_window("main")
                    .expect("window should be labeled main")
                    .is_visible()
                    .unwrap_or(false),
                &app,
            )
            .expect("Failed to toggle visibility"),
            _ => {
                log::warn!("menu item {:?} not handled", event.id);
            }
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } => {
                let _ = show_window(true, &tray.app_handle());
            }
            _ => {}
        })
        .build(&app_handle)?;

    Ok(())
}

pub fn show_window(show: bool, app_handle: &AppHandle) -> Result<()> {
    let window = app_handle.get_webview_window("main").unwrap();
    match show {
        true => {
            window.show()?;
            if window.is_minimized()? {
                window.unminimize()?;
            }
            let _ = window.set_focus();
        }
        false => window.hide()?,
    }
    app_handle
        .tray_by_id(TRAY_ICON_ID)
        .expect("tray icon should exist")
        .set_menu(Some(create_tray_menu(app_handle.clone())?))?;

    Ok(())
}

pub fn init(app_handle: AppHandle) -> Result<()> {
    let store = app_handle.store(".settings.json")?;

    create_system_tray(app_handle.clone())?;

    let start_minimized = store
        .get("start_minimized_state")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if start_minimized {
        show_window(false, &app_handle.clone())?;
    }
    Ok(())
}

pub fn window_event_handler(window: &tauri::Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api, .. } => {
            let _ = show_window(false, window.app_handle());
            api.prevent_close();
        }
        _ => {}
    }
}
