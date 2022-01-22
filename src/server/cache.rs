use lru::LruCache;
use std::sync::{Arc, Mutex};

pub struct Cache {
    cache: Arc<Mutex<LruCache<String, crate::tree::ElevationResult>>>,
}

impl Cache {
    pub fn new(cache_cap: usize) -> Cache {
        let cache = Arc::new(Mutex::new(lru::LruCache::new(cache_cap)));
        return Cache { cache };
    }

    pub fn get(
        &self,
        lat: f64,
        lng: f64,
        dataset_id: Option<String>,
    ) -> Option<crate::tree::ElevationResult> {
        let cache_key = format!("{}:{}:{:?}", lat, lng, dataset_id);
        let mut handle = self.cache.lock().unwrap();
        let cache_res = handle.get(&cache_key);

        if cache_res.is_none() {
            return None;
        }

        let res = cache_res.unwrap();

        Some(res.clone())
    }

    pub fn add(&self, lat: f64, lng: f64, elev: crate::tree::ElevationResult) {
        let cache_key = format!("{}:{}:{:?}", lat, lng, elev.dataset_id);
        let mut handle = self.cache.lock().unwrap();

        handle.put(cache_key, elev);
    }
}
