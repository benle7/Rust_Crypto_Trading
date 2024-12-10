use super::QueryDatabaseTable;
use crate::db::models::TransactionRecord;
use crate::db::traits::error::Result;

#[async_trait::async_trait]
pub trait QueryTransactionRecords: QueryDatabaseTable<TransactionRecord> + Send + Sync {
    async fn add_transaction(&self, transaction: &TransactionRecord) -> Result<()>;
    async fn update_transaction(
        &self,
        old_tx: &TransactionRecord,
        new_tx: &TransactionRecord,
    ) -> Result<()>;
    async fn get_transaction_by_id(
        &self,
        transaction_id: &str,
    ) -> Result<Option<TransactionRecord>>;
    async fn get_user_transactions(
        &self,
        username: &str,
        currency_filter: Option<&str>,
        active_filter: Option<bool>,
    ) -> Result<Vec<TransactionRecord>>;
}
