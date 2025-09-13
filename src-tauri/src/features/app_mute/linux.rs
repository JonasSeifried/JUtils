use crate::error::{Error, Result};

pub fn get_running_apps_with_audio_sessions() -> Result<Vec<String>> {
    // TODO: Implement this for linux.
    Err(Error::Unimplemented("linux app_mute not yet implemented.".to_owned()))
}

pub fn mute_app_by_name(app_name: &str) -> Result<()> {
    // TODO: Implement this for linux.
    Err(Error::Unimplemented("linux app_mute not yet implemented.".to_owned()))
}