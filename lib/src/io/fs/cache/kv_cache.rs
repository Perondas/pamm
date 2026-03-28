use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct KVCache {
    db: sled::Db,
}

impl KVCache {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, anyhow::Error> {
        let db = sled::open(path)?;
        Ok(KVCache { db })
    }

    pub fn get<K: AsRef<[u8]>, V: DeserializeOwned>(
        &self,
        key: K,
    ) -> Result<Option<V>, anyhow::Error> {
        match self.db.get(key)? {
            Some(value) => bincode::serde::decode_from_slice(&value, bincode::config::standard())
                .map(|(s, _)| Some(s))
                .map_err(Into::into)
                .map_err(|e: anyhow::Error| e.context("Failed to decode cached value")),
            None => Ok(None),
        }
    }

    pub fn set<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: V) -> Result<(), anyhow::Error> {
        let encoded: Vec<u8> = bincode::serde::encode_to_vec(value, bincode::config::standard())?;
        self.db.insert(key, encoded)?;
        Ok(())
    }

    pub fn remove(&self, key: &str) -> Result<(), anyhow::Error> {
        self.db.remove(key)?;
        Ok(())
    }

    pub fn remove_all(&self) -> Result<(), anyhow::Error> {
        self.db.clear()?;
        Ok(())
    }
}

impl Drop for KVCache {
    fn drop(&mut self) {
        let _ = self.db.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::TestTempDir;
    use serde::{Deserialize, Serialize};
    use std::fs;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_kv_cache_operations() {
        let temp_path = std::env::temp_dir().join("pamm_test_kv_cache");
        if temp_path.exists() {
            fs::remove_dir_all(&temp_path).ok();
        }
        let _temp = TestTempDir::new(temp_path.clone());

        let cache = KVCache::new(&temp_path).unwrap();

        // 1. Test get on empty
        let empty_val: Option<String> = cache.get("doesntexist").unwrap();
        assert_eq!(empty_val, None);

        // 2. Test set and get
        cache.set("string_key", "hello world".to_string()).unwrap();
        let str_val: Option<String> = cache.get("string_key").unwrap();
        assert_eq!(str_val, Some("hello world".to_string()));

        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        cache.set("struct_key".as_bytes(), &test_data).unwrap();
        let struct_val: Option<TestData> = cache.get("struct_key".as_bytes()).unwrap();
        assert_eq!(struct_val, Some(test_data));

        // 3. Test remove
        cache.remove("string_key").unwrap();
        let removed_val: Option<String> = cache.get("string_key").unwrap();
        assert_eq!(removed_val, None);

        // 4. Test remove_all
        cache.set("k1", 1).unwrap();
        cache.set("k2", 2).unwrap();

        cache.remove_all().unwrap();

        let k1_val: Option<i32> = cache.get("k1").unwrap();
        let k2_val: Option<i32> = cache.get("k2").unwrap();

        assert_eq!(k1_val, None);
        assert_eq!(k2_val, None);
    }
}
