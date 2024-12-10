use crate::coingecko::entities::CurrencyDetails;
use core::panic;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

use super::entities::{CurrencyInfo, CurrencyUsdPrice, ProtocolInfo};

const COINGECKO_URL: &str = "https://api.coingecko.com/api/v3";
const USER_AGENT_KEY: &str = "User-Agent";
const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
    AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";

#[derive(Clone, Debug)]
pub struct CoinGeckoClient {
    client: reqwest::Client,
}

impl CoinGeckoClient {
    pub async fn new() -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.append(USER_AGENT_KEY, HeaderValue::from_static(USER_AGENT_VALUE));
        let client = match reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
        {
            Ok(client) => client,
            Err(_) => panic!("Failed to create a reqwest client"),
        };
        Self { client }
    }

    pub async fn get_supported_currencies(&self) -> Result<Vec<CurrencyDetails>, reqwest::Error> {
        let request = "/coins/list";
        let response = self
            .client
            .get(format!("{COINGECKO_URL}/{request}"))
            .send()
            .await?;
        let supported_currencies = response.json::<Vec<CurrencyDetails>>().await?;
        Ok(supported_currencies)
    }

    pub async fn get_currencies_price(
        &self,
        currencies_names: &[&str],
    ) -> Result<HashMap<String, CurrencyUsdPrice>, reqwest::Error> {
        let request = format!(
            "simple/price?ids={currencies}&vs_currencies=usd",
            currencies = currencies_names.join(",")
        );
        let response = self
            .client
            .get(format!("{COINGECKO_URL}/{request}"))
            .send()
            .await?;
        let currencies_price = response.json::<HashMap<String, CurrencyUsdPrice>>().await?;
        Ok(currencies_price)
    }

    pub async fn get_currencies_market_info(
        &self,
        currencies_names: &[&str],
    ) -> Result<Vec<CurrencyInfo>, reqwest::Error> {
        let request = format!(
            "/coins/markets?vs_currency=usd&ids={currencies} \
            &order=market_cap_desc&price_change_percentage=1h,24h,7d,30d,1y",
            currencies = currencies_names.join(",")
        );
        let response = self
            .client
            .get(format!("{COINGECKO_URL}/{request}"))
            .send()
            .await?;
        let currencies_market_info = response.json::<Vec<CurrencyInfo>>().await?;
        Ok(currencies_market_info)
    }

    pub async fn get_protocol_info_by_name(
        &self,
        protocol_name: &str,
    ) -> Result<ProtocolInfo, reqwest::Error> {
        let request = format!("coins/{protocol_name}?community_data=true&developer_data=true");
        let res = self
            .client
            .get(format!("{COINGECKO_URL}/{request}"))
            .send()
            .await?;
        let protocol_info = res.json::<ProtocolInfo>().await?;
        Ok(protocol_info)
    }
}

#[cfg(test)]
mod tests {
    use crate::coingecko::coingecko_client::CoinGeckoClient;

    #[actix_web::test]
    async fn get_supported_currencies_test() {
        let client = CoinGeckoClient::new().await;
        let supported_currencies = client.get_supported_currencies().await;
        assert!(supported_currencies.is_ok());
        assert!(!supported_currencies.unwrap().is_empty());
    }

    #[actix_web::test]
    async fn get_currencies_price_test() {
        let client = CoinGeckoClient::new().await;
        let currencies = &["bitcoin", "ethereum"];
        let currencies_price = client.get_currencies_price(currencies).await;
        assert!(currencies_price.is_ok());
        assert!(currencies_price.unwrap().len() == currencies.len());
    }

    #[actix_web::test]
    async fn get_currencies_market_info_test() {
        let client = CoinGeckoClient::new().await;
        let currencies = &["bitcoin", "ethereum"];
        let currencies_market_info = client.get_currencies_market_info(currencies).await;
        assert!(currencies_market_info.is_ok());
        assert!(currencies_market_info.unwrap().len() == currencies.len());
    }

    #[actix_web::test]
    async fn get_protocol_info_by_name_test() {
        let client = CoinGeckoClient::new().await;
        let currencies_market_info = client.get_protocol_info_by_name("bitcoin").await;
        assert!(currencies_market_info.is_ok());
    }
}
