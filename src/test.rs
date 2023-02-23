
use super::api_handler::indexer;
use crate::methods::caching::Caching;


use actix_web::{test, web, App, http::StatusCode};
use std::sync::{Arc, RwLock};

#[tokio::test]
async fn test_without_url() {
    let cache = web::Data::new(Arc::new(RwLock::new(Caching::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(indexer))).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}


#[tokio::test]
async fn test_if_ok() {
    let cache = web::Data::new(Arc::new(RwLock::new(Caching::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(indexer))).await;

    let req = test::TestRequest::get().uri("/?url=https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
}


#[tokio::test]
async fn test_if_request_not_received() {
    let cache = web::Data::new(Arc::new(RwLock::new(Caching::new())));

    let mut app = test::init_service(App::new().app_data(cache.clone()).route("/", web::get().to(indexer))).await;

    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}