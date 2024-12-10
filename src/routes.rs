use crate::handlers;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

const CRYPTO_SCOPE: &str = "/crypto";

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(handlers::health::get_health_status)
        .service(handlers::users::register)
        .service(handlers::users::login)
        .service(
            web::scope(CRYPTO_SCOPE)
                .wrap(HttpAuthentication::bearer(handlers::utils::jwt_validator))
                .service(handlers::currencies::get_supported_currencies)
                .service(handlers::currencies::get_protocol_info_by_name)
                .service(handlers::currencies::get_currencies_market_info)
                .service(handlers::transactions::open_transaction)
                .service(handlers::transactions::close_transaction)
                .service(handlers::transactions::get_transactions),
        );
}
