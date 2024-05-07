use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut lock = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = lock.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow!(e.to_string()))?
            .clone())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        let reader = self.data.read().map_err(|_e| fmt::Error {})?;
        for (key, value) in reader.iter() {
            writeln!(f, "   {:?}: {}", key, value)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
