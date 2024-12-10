use derive_error::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(non_std, no_from)]
    InternalDb(mongodb::error::Error),
    #[error(non_std, no_from)]
    Unknown(anyhow::Error),
}

impl From<mongodb::error::Error> for DatabaseError {
    fn from(e: mongodb::error::Error) -> Self {
        Self::InternalDb(e)
    }
}

impl From<anyhow::Error> for DatabaseError {
    fn from(e: anyhow::Error) -> Self {
        Self::Unknown(e)
    }
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
