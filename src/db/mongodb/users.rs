use super::{MongoDB, MongoModel};
use crate::db::traits::error::Result;
use crate::db::{models::UserRecord, traits::users::QueryUserRecords};

impl MongoModel for UserRecord {
    fn collection_name() -> &'static str {
        "users"
    }
}

#[async_trait::async_trait]
impl QueryUserRecords for MongoDB {
    async fn add_user(&self, user: &UserRecord) -> Result<()> {
        Ok(self.insert_one(user).await?)
    }

    async fn find_user(&self, username: &str) -> Result<Option<UserRecord>> {
        Ok(self.find_one(&[("username", username)]).await?)
    }
}
