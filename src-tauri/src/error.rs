#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SQLError(#[from] rusqlite::Error),

    #[cfg(windows)]
    #[error(transparent)]
    WinowsError(#[from] windows::core::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
