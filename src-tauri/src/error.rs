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

    #[error("{0}")]
    UnexpectedError(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
