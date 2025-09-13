use crate::error::{Result, Error};


pub fn toggle_mic() -> Result<bool> {
    // TODO: Implement microphone toggle for Linux.
    Err(Error::Unimplemented("macOs mic_mute not yet implemented.".to_owned()))
}
