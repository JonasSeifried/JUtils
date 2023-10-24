use crate::{
    db,
    error::Error,
    features::{auto_launch, mic_mute},
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
pub fn toggle_mic() -> Result<(), Error> {
    let new_state = db::toggle_mute_state()?;
    mic_mute::toggle_mic(new_state)
}

#[tauri::command]
pub fn set_auto_launch(new_state: bool) -> Result<(), Error> {
    auto_launch::set_auto_launch(new_state)
}

#[tauri::command]
pub fn get_auto_launch() -> Result<bool, Error> {
    db::get_auto_launch()
}
