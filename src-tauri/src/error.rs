#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    SQLError(#[from] rusqlite::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub trait ToError {
    fn to_error(self) -> Error
    where
        Self: Sized;
}

impl ToError for rusqlite::Error {
    fn to_error(self) -> Error {
        Error::SQLError(self)
    }
}
