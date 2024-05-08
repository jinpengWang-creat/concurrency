use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt::Display,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        self.data
            .get(key.as_ref())
            .map(|counter| {
                counter.fetch_add(1, Ordering::Relaxed);
            })
            .ok_or(anyhow!(format!("key: {} is not exist!", key.as_ref())))
    }
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for (key, value) in self.data.iter() {
            writeln!(f, "   {:?}: {}", key, value.load(Ordering::Relaxed))?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
