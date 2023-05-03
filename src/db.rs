use std::path::PathBuf;
use std::time::{Duration, Instant};

use async_std::sync::RwLock;
use tide::log::{debug, info};

use crate::data::*;
use crate::load::load_data;

pub struct Database {
    db: RwLock<Data>,
    path: PathBuf,
    interval: Duration,
    last_refreshed: RwLock<Option<Instant>>,
}

impl Database {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database {
            db: RwLock::new(Data::empty()),
            path: path.into(),
            interval: Duration::from_secs(1),
            last_refreshed: RwLock::new(None),
        }
    }

    pub async fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut Data),
    {
        let data = &mut *self.db.write().await;
        f(data)
    }

    pub async fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Data) -> R,
    {
        f(&*self.db.read().await)
    }

    pub async fn refresh(&self) {
        let now = Instant::now();

        let elapsed = self
            .last_refreshed
            .read()
            .await
            .map(|lr| now.duration_since(lr));

        *self.last_refreshed.write().await = Some(now);

        match elapsed {
            Some(elapsed) if elapsed >= self.interval => self.force_refresh().await,
            None => self.force_refresh().await,
            _ => debug!("Not refreshing yet"),
        }
    }

    pub async fn force_refresh(&self) {
        info!("Refreshing content...");

        let version = self.db.read().await.version;

        let mut new_data = load_data(&self.path).await;
        new_data.version = version + 1;

        self.modify(|data| {
            *data = new_data;
        })
        .await;
    }
}
