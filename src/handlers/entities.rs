use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RegisterRequest {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CurrenciesMarketRequest {
    pub currencies: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct OpenTransactionRequest {
    pub username: String,
    pub currency: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CloseTransactionRequest {
    pub username: String,
    pub transaction_id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GetTransactionsRequest {
    pub username: String,
    pub currency_filter: Option<String>,
    pub active_filter: Option<bool>,
}
