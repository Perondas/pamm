use anyhow::anyhow;
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
