use crate::{
    db,
    error::Error,
    features::mic_mute,
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
