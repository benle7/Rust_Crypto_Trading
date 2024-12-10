use mongodb::bson;

use super::{MongoDB, MongoModel};
use crate::db::traits::error::Result;
use crate::db::{models::TransactionRecord, traits::transactions::QueryTransactionRecords};

impl MongoModel for TransactionRecord {
    fn collection_name() -> &'static str {
        "transactions"
    }
}

#[async_trait::async_trait]
impl QueryTransactionRecords for MongoDB {
    async fn add_transaction(&self, transaction: &TransactionRecord) -> Result<()> {
        Ok(self.insert_one(transaction).await?)
    }

    async fn update_transaction(
        &self,
        old_tx: &TransactionRecord,
        new_tx: &TransactionRecord,
    ) -> Result<()> {
        Ok(self.update_one(old_tx, new_tx).await?)
    }

    async fn get_transaction_by_id(
        &self,
        transaction_id: &str,
    ) -> Result<Option<TransactionRecord>> {
        Ok(self.find_one(&[("transaction_id", transaction_id)]).await?)
    }

    async fn get_user_transactions(
        &self,
        username: &str,
        currency_filter: Option<&str>,
        active_filter: Option<bool>,
    ) -> Result<Vec<TransactionRecord>> {
        let mut query = bson::doc! {};
        query.insert("username", bson::doc! {"$eq": username});
        if let Some(currency) = currency_filter {
            query.insert("currency", bson::doc! {"$eq": currency});
        }
        if let Some(active) = active_filter {
            query.insert("active", bson::doc! {"$eq": active});
        }
        Ok(self.find_many(query).await?)
    }
}
