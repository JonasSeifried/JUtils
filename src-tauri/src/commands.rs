use log::info;
use tauri_plugin_store::StoreExt;

use crate::{
    error::{Error, Result},
    features::{app_mute, audio_manager, mic_mute},
};

#[tauri::command]
pub fn toggle_mic(app_handle: tauri::AppHandle) -> Result<()> {
    let new_state = mic_mute::toggle_mic()?;
    info!("Toggled Mic -> {}", new_state);

    // Play sound when mic toggled

    let store = app_handle.store(".settings.json")?;
    let volume = store
        .get("mic_mute_audio_volume")
        .and_then(|v| {
            v.as_f64()
                .or_else(|| v.as_str().and_then(|s| s.parse::<f64>().ok()))
        })
        .ok_or(Error::UnexpectedError(
            "Failed to get mic mute audio volume".to_string(),
        ))? as f32;
    audio_manager::play_mute_sound(new_state, volume / 100.0)
}

#[tauri::command]
pub fn get_running_apps() -> Result<Vec<String>> {
    app_mute::get_running_apps_with_audio_sessions()
}

#[tauri::command]
pub fn toggle_app_mute(app_name: String) -> Result<()> {
    app_mute::mute_app_by_name(&app_name)
}
