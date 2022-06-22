use lru::LruCache;
use std::sync::{Arc, Mutex};

pub struct Cache {
    cache: Arc<Mutex<LruCache<String, crate::tree::ElevationResult>>>,
}

impl Cache {
    /// Build a new cache with given capacity
    pub fn new(cache_cap: usize) -> Cache {
        let cache = Arc::new(Mutex::new(lru::LruCache::new(cache_cap)));
        return Cache { cache };
    }

    /// Get entry from cache
    pub fn get(
        &self,
        lat: f64,
        lng: f64,
        dataset_id: Option<String>,
    ) -> Option<crate::tree::ElevationResult> {
        let cache_key = format!("{}:{}:{:?}", lat, lng, dataset_id);
        let mut handle = self.cache.lock().unwrap();
        let cache_res = handle.get(&cache_key);

        match cache_res {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    /// Add entry with the given key and value to cache
    pub fn add(&self, lat: f64, lng: f64, elev: crate::tree::ElevationResult) {
        let cache_key = format!("{}:{}:{:?}", lat, lng, elev.dataset_id);
        let mut handle = self.cache.lock().unwrap();
        handle.put(cache_key, elev);
    }
}
