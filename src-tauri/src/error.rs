pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SQLError(#[from] rusqlite::Error),

    #[cfg(windows)]
    #[error(transparent)]
    WinowsError(#[from] windows::core::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    AutoLaunchError(#[from] auto_launch::Error),

    #[error(transparent)]
    AudioStreamError(#[from] rodio::StreamError),

    #[error(transparent)]
    AudioDecoderError(#[from] rodio::decoder::DecoderError),

    #[error(transparent)]
    AudioPlayError(#[from] rodio::PlayError),

    #[error(transparent)]
    GlobalHotKeyError(#[from] global_hotkey::Error),

    #[error("{0}")]
    UnexpectedError(String),

    #[error("'{0}' is not an accepted key")]
    UnacceptedKey(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
