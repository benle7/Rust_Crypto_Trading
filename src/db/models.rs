use super::traits::DatabaseModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRecord {
    pub username: String,
    pub hashed_password: String,
}

impl DatabaseModel for UserRecord {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionRecord {
    pub transaction_id: String,
    pub username: String,
    pub currency: String,
    pub amount: u32,
    pub open_rate: f64,
    pub close_rate: Option<f64>,
    pub active: bool,
}

impl DatabaseModel for TransactionRecord {}
