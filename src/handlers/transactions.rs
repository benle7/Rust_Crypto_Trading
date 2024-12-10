use crate::coingecko::coingecko_client::CoinGeckoClient;
use crate::db::models::TransactionRecord;
use crate::db::mongodb::MongoDB;
use crate::db::traits::transactions::QueryTransactionRecords;
use crate::db::traits::users::QueryUserRecords;
use crate::handlers::entities::{
    CloseTransactionRequest, GetTransactionsRequest, OpenTransactionRequest,
};
use crate::handlers::utils;
use actix_web::{http::header::ContentType, post, web, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;

const TX_ID_LEN: usize = 16;

#[post("/transactions/open")]
pub async fn open_transaction(
    db_client: web::Data<MongoDB>,
    coingecko_client: web::Data<CoinGeckoClient>,
    req: web::Json<OpenTransactionRequest>,
    credentials: BearerAuth,
) -> HttpResponse {
    let username = req.username.clone();
    if !utils::validate_user_jwt(credentials, username.clone()).await {
        return HttpResponse::Unauthorized().finish();
    }
    let maybe_user = db_client.find_user(username.as_str()).await;
    match maybe_user {
        Ok(Some(_)) => {
            let tx_id = utils::generate_random_string(TX_ID_LEN);
            let currency = req.currency.clone();
            let price_res = match coingecko_client
                .get_currencies_price(&[currency.as_str()])
                .await
            {
                Ok(price) => price,
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Failed to open transaction")
                }
            };
            let open_rate = price_res.get(&currency).unwrap().usd;
            let transaction = TransactionRecord {
                transaction_id: tx_id,
                username: req.username.clone(),
                currency,
                amount: req.amount,
                open_rate,
                close_rate: None,
                active: true,
            };
            match db_client.add_transaction(&transaction).await {
                Ok(_) => HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(transaction),
                Err(_) => HttpResponse::InternalServerError().body("Failed to open transaction"),
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("Username is not exists"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to open transaction"),
    }
}

#[post("/transactions/close")]
pub async fn close_transaction(
    db_client: web::Data<MongoDB>,
    coingecko_client: web::Data<CoinGeckoClient>,
    req: web::Json<CloseTransactionRequest>,
    credentials: BearerAuth,
) -> HttpResponse {
    let username = req.username.clone();
    if !utils::validate_user_jwt(credentials, username.clone()).await {
        return HttpResponse::Unauthorized().finish();
    }
    let maybe_user = db_client.find_user(username.as_str()).await;
    match maybe_user {
        Ok(Some(_)) => {
            let transaction = match db_client
                .get_transaction_by_id(req.transaction_id.as_str())
                .await
            {
                Ok(Some(tx)) => {
                    if tx.username != username {
                        return HttpResponse::BadRequest().body("transaction is not exists");
                    }
                    tx
                }
                Ok(None) => return HttpResponse::BadRequest().body("transaction is not exists"),
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Failed to close transaction")
                }
            };
            let currency = transaction.currency.clone();
            let price_res = match coingecko_client
                .get_currencies_price(&[currency.as_str()])
                .await
            {
                Ok(price) => price,
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Failed to close transaction")
                }
            };
            let close_rate = price_res.get(&currency).unwrap().usd;
            let mut new_transaction = transaction.clone();
            new_transaction.close_rate = Some(close_rate);
            new_transaction.active = false;
            match db_client
                .update_transaction(&transaction, &new_transaction)
                .await
            {
                Ok(_) => HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(new_transaction),
                Err(_) => HttpResponse::InternalServerError().body("Failed to close transaction"),
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("Username is not exists"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to close transaction"),
    }
}

#[post("/transactions")]
pub async fn get_transactions(
    db_client: web::Data<MongoDB>,
    req: web::Json<GetTransactionsRequest>,
    credentials: BearerAuth,
) -> HttpResponse {
    let username = req.username.clone();
    if !utils::validate_user_jwt(credentials, username.clone()).await {
        return HttpResponse::Unauthorized().finish();
    }
    let maybe_user = db_client.find_user(username.as_str()).await;
    match maybe_user {
        Ok(Some(_)) => {
            let currency_filter = req.currency_filter.as_deref();
            let transactions = db_client
                .get_user_transactions(req.username.as_str(), currency_filter, req.active_filter)
                .await;
            match transactions {
                Ok(txs) => HttpResponse::Ok()
                    .content_type(ContentType::json())
                    .json(txs),
                Err(_) => HttpResponse::InternalServerError().body("Failed to get transactions"),
            }
        }
        Ok(None) => HttpResponse::BadRequest().body("Username is not exists"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to get transactions"),
    }
}
