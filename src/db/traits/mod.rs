use serde::{de::DeserializeOwned, Serialize};

pub mod error;
pub mod transactions;
pub mod users;

pub trait DatabaseModel
where
    Self: Clone,
    Self: Send + Sync,
    Self: Serialize + DeserializeOwned + Unpin,
{
}

pub trait QueryDatabaseTable<T>
where
    T: DatabaseModel,
{
}
