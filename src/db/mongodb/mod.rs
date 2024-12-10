use futures_util::TryStreamExt;
use mongodb::bson::Document;
use mongodb::error::Result;
use mongodb::{bson, Collection, Database};

use super::traits::{DatabaseModel, QueryDatabaseTable};

pub mod transactions;
pub mod users;

#[derive(Clone, Debug)]
pub struct MongoDB {
    db: Database,
}

trait MongoModel: DatabaseModel {
    fn collection_name() -> &'static str;
}

impl MongoDB {
    pub async fn new(connection_string: &str, db_schema_name: &str) -> anyhow::Result<Self> {
        let client = mongodb::Client::with_uri_str(connection_string).await?;
        Ok(Self {
            db: client.database(db_schema_name),
        })
    }

    fn collection<T: MongoModel>(&self) -> Collection<T> {
        self.db.collection(T::collection_name())
    }

    async fn find_many<T: MongoModel>(&self, query: Document) -> Result<Vec<T>> {
        let cursor = self.collection::<T>().find(query, None).await?;
        let items = cursor.try_collect::<Vec<T>>().await?;
        Ok(items)
    }

    async fn find_one<T: MongoModel>(&self, key_value_pairs: &[(&str, &str)]) -> Result<Option<T>> {
        let mut query = bson::doc! {};
        for (key, value) in key_value_pairs {
            query.insert(*key, bson::doc! {"$eq": *value});
        }
        let item = self.collection().find_one(query, None).await?;
        Ok(item)
    }

    async fn update_one<T: MongoModel>(&self, old_item: &T, updated_item: &T) -> Result<()> {
        let filter = bson::to_document(old_item)?;
        let update_doc = bson::to_document(updated_item)?;
        let update = bson::doc! { "$set": update_doc };
        self.collection::<T>()
            .update_one(filter, update, None)
            .await?;
        Ok(())
    }

    async fn insert_one<T: MongoModel>(&self, item: &T) -> Result<()> {
        self.collection::<T>().insert_one(item, None).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<T: MongoModel> QueryDatabaseTable<T> for MongoDB {}
