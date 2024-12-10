use coingecko::response::common::{CommunityData, DeveloperData};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyDetails {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProtocolInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub block_time_in_minutes: f64,
    pub hashing_algorithm: Value,
    pub categories: Vec<String>,
    pub genesis_date: Value,
    pub market_cap_rank: Value,
    pub community_data: Option<CommunityData>,
    pub developer_data: Option<DeveloperData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyUsdPrice {
    pub usd: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyInfo {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: Option<f64>,
    pub market_cap: Option<f64>,
    pub market_cap_rank: Value,
    #[serde(rename = "high_24h")]
    pub high24_h: Option<f64>,
    #[serde(rename = "low_24h")]
    pub low24_h: Option<f64>,
    pub circulating_supply: Option<f64>,
    pub total_supply: Option<f64>,
    pub max_supply: Option<f64>,
    #[serde(rename = "price_change_percentage_1h_in_currency")]
    pub price_change_percentage1_h_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_1y_in_currency")]
    pub price_change_percentage1_y_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_24h_in_currency")]
    pub price_change_percentage24_h_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_30d_in_currency")]
    pub price_change_percentage30_d_in_currency: Option<f64>,
    #[serde(rename = "price_change_percentage_7d_in_currency")]
    pub price_change_percentage7_d_in_currency: Option<f64>,
}
