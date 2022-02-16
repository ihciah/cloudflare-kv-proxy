use std::{ops::Add, sync::Arc};

use coarsetime::{Duration, Instant};
use hashlink::LruCache;
use parking_lot::Mutex;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Client, Result};

#[derive(Debug)]
pub(crate) struct Cache {
    expire_ttl: Duration,
    inner: Mutex<LruCache<String, Arc<CacheValue>>>,
}

#[derive(Debug, Clone)]
pub(crate) struct CacheValue {
    expire: Instant,
    value: String,
}

pub(crate) fn new_cache(cache_size: usize, expire_ttl: Duration) -> Cache {
    Cache {
        expire_ttl,
        inner: Mutex::new(LruCache::new(cache_size)),
    }
}

impl Client {
    /// Get value of a key.
    pub async fn get<T: DeserializeOwned + Serialize>(&self, key: &str) -> Result<T> {
        // try read cache
        if let Some(cv) = self.get_cache(key) {
            if let Ok(v) = serde_json::from_str(&cv.value) {
                return Ok(v);
            }
        }
        // raw get
        let r = execute!(self.client.get(format!("{}{}", self.endpoint, key)));
        // write cache only if success
        if let Ok(r) = &r {
            self.set_cache(key, r);
        }
        r
    }

    /// Clear all cached values.
    pub fn clear_cached(&self) {
        self.cache.inner.lock().clear()
    }

    /// Delete a single key value from cache only.
    pub fn prune_cached(&self, key: &str) {
        self.cache.inner.lock().remove(key);
    }

    pub(crate) fn get_cache(&self, key: &str) -> Option<Arc<CacheValue>> {
        let now = Instant::now();
        let mut locked = self.cache.inner.lock();
        let cv_ref = locked.get(key)?;
        if cv_ref.expire < now {
            return None;
        }
        let cv = cv_ref.to_owned();
        Some(cv)
    }

    pub(crate) fn set_cache<T: Serialize + ?Sized>(&self, key: &str, data: &T) {
        if let Ok(j) = serde_json::to_string(data) {
            let cv = CacheValue {
                expire: Instant::now().add(self.cache.expire_ttl),
                value: j,
            };
            self.cache.inner.lock().insert(key.to_owned(), Arc::new(cv));
        }
    }
}
