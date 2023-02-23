use serde::{Deserialize};
use reqwest;

use std::collections::HashMap;
use std::time::{SystemTime};

#[derive(Debug, Deserialize)]
pub struct CacheServer {
    url: String,
}

impl CacheServer {
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Debug)]
pub struct TempResponse {
    header: reqwest::header::HeaderMap,
    status: reqwest::StatusCode,
    body: Vec<u8>,
    expiry: SystemTime,
}

#[derive(Debug)]
pub struct Caching {
    data: HashMap<String, TempResponse>,
}

impl Caching {
    pub fn new() -> Self {
        Caching {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&TempResponse> {
        self.data.get(key)
    }

    pub fn put(&mut self, key: &str, cached_response: TempResponse) {
        self.data.insert(key.to_string(), cached_response);
    }

    pub fn remove_expired_entries(&mut self) {
        let now = SystemTime::now();
        self.data.retain(|_, value| value.get_expiry() > now);
    }
}

impl TempResponse {
    pub fn new(
        header: reqwest::header::HeaderMap, 
        status: reqwest::StatusCode,
        body: Vec<u8>,
        expiry: SystemTime
    ) -> Self {
        TempResponse {
            header,
            status,
            body,
            expiry
        }
    }

    pub fn get_body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn get_header(&self) -> reqwest::header::HeaderMap {
        self.header.clone()
    }

    pub fn get_status(&self) -> reqwest::StatusCode {
        self.status
    }

    pub fn get_expiry(&self) -> SystemTime {
        self.expiry
    }
}
