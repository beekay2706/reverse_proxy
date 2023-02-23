
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};

use super::caching::{CacheServer, Caching, TempResponse};

use actix_web::{web, error, Result, HttpResponse};
use log::{debug, info};
use reqwest;

const TTL_CACHE: u64 = 30;

pub async fn indexer(cache_server: web::Query<CacheServer>, caching: web::Data<Arc<RwLock<Caching>>>) -> Result<HttpResponse, actix_web::Error> {
    let key = cache_server.into_inner().get_url();
    let mut caching = caching.write().unwrap();
    info!("Cache Server: {}", key);

    //checking if the response is already in the Caching
    if let Some(cached_response) = caching.get(&key) {
        if cached_response.get_expiry() > SystemTime::now() {
            debug!("URL found in Caching.");
            let mut headers = HttpResponse::build(cached_response.get_status());
            for (header_name, header_value) in cached_response.get_header().iter() {
                headers.insert_header((header_name, header_value));
            }
            return Ok(headers.body(cached_response.get_body()));
        }
    } 

    debug!("Querying the origin Server");
    let response = match reqwest::get(&key).await {
        Ok(resp) => resp,
        Err(e) => return Err(error::ErrorInternalServerError(format!("Error submitting request: {}", e))),
    };

    let cloned_headers = response.headers().clone();
    let status = response.status();
    let body_bytes = match response.bytes().await {
        Ok(bytes) => bytes,
        Err(e) => return Err(error::ErrorInternalServerError(format!("Error reading the response: {}", e))),
    };
    let body = body_bytes.to_vec();

    let expiration = SystemTime::now() + Duration::from_secs(TTL_CACHE);
    let cached_response = TempResponse::new(cloned_headers.clone(), status, body.clone(), expiration);
    caching.put(&key, cached_response);
    caching.remove_expired_entries();

    let mut headers = HttpResponse::build(status);
    for (header_name, header_value) in cloned_headers.iter() {
        headers.insert_header((header_name.clone(), header_value.clone()));
    }
    Ok(headers.body(body))
}