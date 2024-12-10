use crate::{
    coingecko::coingecko_client::CoinGeckoClient, handlers::entities::CurrenciesMarketRequest,
};
use actix_web::{get, http::header::ContentType, web, HttpResponse};

#[get("/currencies")]
pub async fn get_supported_currencies(
    coingecko_client: web::Data<CoinGeckoClient>,
) -> HttpResponse {
    let supported_currencies = match coingecko_client.get_supported_currencies().await {
        Ok(supported_currencies) => supported_currencies,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to get supported currencies")
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(supported_currencies)
}

#[get("/currencies/protocol/{currency}")]
pub async fn get_protocol_info_by_name(
    coingecko_client: web::Data<CoinGeckoClient>,
    currency: web::Path<String>,
) -> HttpResponse {
    let protocol_info = match coingecko_client
        .get_protocol_info_by_name(currency.as_str())
        .await
    {
        Ok(protocol_info) => protocol_info,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get protocol info"),
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(protocol_info)
}

#[get("/currencies/market")]
pub async fn get_currencies_market_info(
    coingecko_client: web::Data<CoinGeckoClient>,
    currencies_market_request: web::Json<CurrenciesMarketRequest>,
) -> HttpResponse {
    let currencies_names: Vec<&str> = currencies_market_request
        .currencies
        .iter()
        .map(|s| s.as_str())
        .collect();
    let currencies_market_info = match coingecko_client
        .get_currencies_market_info(&currencies_names)
        .await
    {
        Ok(currencies_market_info) => currencies_market_info,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to get currencies market info")
        }
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(currencies_market_info)
}
