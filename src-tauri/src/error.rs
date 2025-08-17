#[cfg(windows)]
use std::string::FromUtf16Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    TauriError(#[from] tauri::Error),

    #[error(transparent)]
    TauriStoreError(#[from] tauri_plugin_store::Error),

    #[cfg(windows)]
    #[error(transparent)]
    WindowsError(#[from] windows_result::Error),

    #[cfg(windows)]
    #[error(transparent)]
    FromUtf16Error(#[from] FromUtf16Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AutoLaunchError(#[from] tauri_plugin_autostart::Error),

    #[error(transparent)]
    AudioStreamError(#[from] rodio::StreamError),

    #[error(transparent)]
    AudioDecoderError(#[from] rodio::decoder::DecoderError),

    #[error(transparent)]
    AudioPlayError(#[from] rodio::PlayError),

    // #[error(transparent)]
    // GlobalHotKeyError(#[from] global_hotkey::Error),

    // #[error(transparent)]
    // AppDirsError(#[from] app_dirs2::AppDirsError),
    #[error("{0}")]
    UnexpectedError(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
