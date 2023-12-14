use log::info;

use crate::{
    db,
    error::Result,
    features::{
        audio_manager,
        hotkey::{Hotkey, MICMUTE},
        mic_mute,
    },
};

#[tauri::command]
pub fn fetch_mic_mute_hotkey() -> Result<Vec<String>> {
    Ok(db::fetch_hotkey(MICMUTE)?.keys)
}

#[tauri::command]
pub fn set_mic_mute_hotkey(
    keys: Vec<String>,
    hotkey_state: tauri::State<'_, crate::features::hotkey::HotkeyState>,
) -> Result<()> {
    hotkey_state
        .inner()
        .0
        .blocking_lock()
        .register_hotkey(MICMUTE, keys.clone())?;
    db::set_hotkey(Hotkey {
        name: MICMUTE.to_string(),
        keys: keys,
    })
}

#[tauri::command]
pub fn toggle_mic() -> Result<()> {
    let new_state = db::toggle_mute_state()?;
    mic_mute::toggle_mic(new_state)?;
    info!("Toggled Mic -> {}", new_state);
    audio_manager::play_mute_sound(new_state)
}

#[tauri::command]
pub fn set_mic_mute_audio_volume(volume: i32) -> Result<()> {
    info!("Set Mic Mute Audio Volume -> {}%", volume);
    db::set_mic_mute_audio_volume(volume as f32 / 100.0)
}

#[tauri::command]
pub fn get_mic_mute_audio_volume() -> Result<i32> {
    let volume = db::fetch_mic_mute_audio_volume()?;
    Ok((volume * 100.0) as i32)
}

#[tauri::command]
pub fn set_start_minimized_state(new_state: bool) -> Result<()> {
    db::set_start_minimized_state(new_state)
}

#[tauri::command]
pub fn fetch_start_minimized_state() -> Result<bool> {
    db::fetch_start_minimized_state()
}
