use crate::{
    db,
    error::Error,
    features::{audio_manager, auto_launch, mic_mute},
    hotkey::{Hotkey, MICMUTE},
};

#[tauri::command]
pub fn fetch_mic_mute_hotkey() -> Result<String, Error> {
    Ok(db::fetch_hotkey(MICMUTE)?.keys)
}

#[tauri::command]
pub fn set_mic_mute_hotkey(keys: &str) -> Result<(), Error> {
    db::set_hotkey(Hotkey {
        name: MICMUTE.to_string(),
        keys: keys.to_string(),
    })
}

#[tauri::command]
pub async fn toggle_mic() -> Result<(), Error> {
    let new_state = db::toggle_mute_state()?;
    mic_mute::toggle_mic(new_state)?;
    println!("Debug: Toggled Mic -> {}", new_state);
    audio_manager::play_mute_sound(new_state).await
}

#[tauri::command]
pub fn set_auto_launch(new_state: bool) -> Result<(), Error> {
    auto_launch::set_auto_launch(new_state)
}

#[tauri::command]
pub fn get_auto_launch() -> Result<bool, Error> {
    db::get_auto_launch()
}

#[tauri::command]
pub fn set_mic_mute_audio_volume(volume: i32) -> Result<(), Error> {
    println!("Debug: Set Mic Mute Audio Volume -> {}%", volume);
    db::set_mic_mute_audio_volume(volume as f32 / 100.0)
}

#[tauri::command]
pub fn get_mic_mute_audio_volume() -> Result<i32, Error> {
    let volume = db::get_mic_mute_audio_volume()?;
    Ok((volume * 100.0) as i32)
}
