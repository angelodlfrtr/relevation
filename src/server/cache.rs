use lru::LruCache;
use std::sync::{Arc, Mutex};

pub struct Cache {
    cache: Arc<Mutex<LruCache<String, crate::ntree::ElevationResult>>>,
}

impl Cache {
    pub fn new(cache_cap: usize) -> Cache {
        let cache = Arc::new(Mutex::new(lru::LruCache::new(cache_cap)));
        return Cache { cache };
    }

    pub fn get<'a>(
        &'a self,
        lat: f64,
        lng: f64,
        dataset_id: Option<String>,
    ) -> Option<&'a crate::ntree::ElevationResult> {
        let cache_key = format!("{}:{}:{:?}", lat, lng, dataset_id);
        let local_cache = self.cache.clone();
        let mut handle = local_cache.lock().unwrap();

        let cache_res = handle.get(&cache_key);
        if cache_res.is_some() {
            return cache_res;
        }

        None
    }

    pub fn add(&self, lat: f64, lng: f64, elev: crate::ntree::ElevationResult) {
        let cache_key = format!("{}:{}:{:?}", lat, lng, elev.dataset_id);
        let local_cache = self.cache.clone();
        let mut handle = local_cache.lock().unwrap();

        handle.put(cache_key, elev);
    }
}
