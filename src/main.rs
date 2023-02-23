use actix_web::{web, App, HttpServer};
use std::sync::{Arc, RwLock};

mod methods;
use methods::api_handler;
use methods::caching::*;

use log4rs;
use serde_yaml;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config_str = include_str!("log.yml");
    let config = serde_yaml::from_str(config_str).unwrap();
    log4rs::init_raw_config(config).unwrap();

    let cache = Arc::new(RwLock::new(Caching::new()));

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(cache.clone()))
        .route("/", web::get().to(api_handler::indexer))
    })
    .bind(("127.0.0.1", 9000))?
    .run()
    .await
}