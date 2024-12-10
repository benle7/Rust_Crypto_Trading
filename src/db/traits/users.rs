use super::QueryDatabaseTable;
use crate::db::models::UserRecord;
use crate::db::traits::error::Result;

#[async_trait::async_trait]
pub trait QueryUserRecords: QueryDatabaseTable<UserRecord> + Send + Sync {
    async fn add_user(&self, user: &UserRecord) -> Result<()>;
    async fn find_user(&self, username: &str) -> Result<Option<UserRecord>>;
}
