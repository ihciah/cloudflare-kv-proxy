mod types;
use reqwest::{header, Response};
use serde::Serialize;
use std::time::Duration;
use types::ApiResult;
pub use types::{Error, NotFoundMapping, Result};

macro_rules! execute {
    ($send: expr) => {
        $send
            .send()
            .await
            .and_then(crate::Response::error_for_status)?
            .json::<crate::ApiResult<_>>()
            .await?
            .into()
    };
}

#[cfg(feature = "cache")]
mod cache;

/// KV Client.
#[derive(Debug)]
pub struct Client {
    endpoint: String,
    client: reqwest::Client,

    #[cfg(feature = "cache")]
    cache: cache::Cache,
}

/// Error when creating KV Client.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("token format error {0}")]
    Token(#[from] reqwest::header::InvalidHeaderValue),
    #[error("client build error {0}")]
    Client(#[from] reqwest::Error),
}

impl Client {
    /// Create client with endpoint and token.
    /// If cache is enabled, you may set cache size and ttl.
    pub fn new<T: Into<String>, E: Into<String>>(
        endpoint: E,
        token: T,
        #[cfg(feature = "cache")] cache_size: usize,
        #[cfg(feature = "cache")] expire_ttl: std::time::Duration,
    ) -> std::result::Result<Self, ClientError> {
        // normalize endpoint
        let mut endpoint: String = endpoint.into();
        if !endpoint.ends_with('/') {
            endpoint.push('/');
        }
        let token = token.into();
        let mut headers = header::HeaderMap::new();
        headers.insert("Authorization", header::HeaderValue::from_str(&token)?);
        Ok(Self {
            endpoint,
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
            #[cfg(feature = "cache")]
            cache: cache::new_cache(cache_size, expire_ttl.into()),
        })
    }

    /// Get value of a key.
    #[cfg(not(feature = "cache"))]
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<T> {
        execute!(self.client.get(format!("{}{}", self.endpoint, key)))
    }

    /// Set a key value pair.
    pub async fn put<T: Serialize + ?Sized>(&self, key: &str, value: &T) -> Result<()> {
        let r: Result<()> = execute!(self
            .client
            .put(format!("{}{}", self.endpoint, key))
            .json(value));
        #[cfg(feature = "cache")]
        if r.is_ok() {
            self.set_cache(key, value);
        }
        r
    }

    /// Set a key value pair with ttl.
    pub async fn put_with_ttl<T: Serialize + ?Sized>(
        &self,
        key: &str,
        value: &T,
        ttl: Duration,
    ) -> Result<()> {
        let r: Result<()> = execute!(self
            .client
            .put(format!("{}{}", self.endpoint, key))
            .header("ttl", ttl.as_secs())
            .json(value));
        #[cfg(feature = "cache")]
        if r.is_ok() {
            self.set_cache(key, value);
        }
        r
    }

    /// Delete a key value pair.
    pub async fn delete(&self, key: &str) -> Result<()> {
        let r: Result<()> = execute!(self.client.delete(format!("{}{}", self.endpoint, key)));
        #[cfg(feature = "cache")]
        if r.is_ok() {
            self.prune_cached(key);
        }
        r
    }
}
