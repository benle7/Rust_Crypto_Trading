use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use coingecko::coingecko_client::CoinGeckoClient;
use db::mongodb::MongoDB;
use std::env;

mod coingecko;
mod config;
mod db;
mod handlers;
mod routes;

const CONFIG_FILE: &str = "config.yml";

use shadow_rs::shadow;
shadow!(build);

pub fn print_shadow_details() {
    println!(
        "
        PROJECT_NAME: {}
        BRANCH: {}
        COMMIT_HASH: {}
        COMMIT_DATE: {}
        COMMIT_AUTHOR: {}
        BUILD_TIME: {}
        ",
        build::PROJECT_NAME,
        build::BRANCH,
        build::COMMIT_HASH,
        build::COMMIT_DATE,
        build::COMMIT_AUTHOR,
        build::BUILD_TIME
    );
}

#[actix_web::main]
pub async fn lib_main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    print_shadow_details();
    let config = config::load_config(CONFIG_FILE);
    dbg!(&config);

    let db_client = MongoDB::new(
        &config.database.connection_string,
        &config.database.schema_name,
    )
    .await
    .expect("Failed to create MongoDB client");
    let coingecko_client = CoinGeckoClient::new().await;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(db_client.clone()))
            .app_data(Data::new(coingecko_client.clone()))
            .configure(routes::routes)
    })
    .bind(&config.server.ip_port)
    .unwrap_or_else(|_| panic!("Failed to bind server to {:?}", &config.server.ip_port));

    println!("Listening on http://{} ...", &config.server.ip_port);
    server.run().await
}
