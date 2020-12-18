use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::{
    path::{Path, PathBuf},
    sync::RwLock,
};

use crate::data::*;
use crate::load::load_data;

pub struct Database {
    db: Arc<RwLock<Data>>,
    path: PathBuf,
    last_call: Arc<RwLock<Option<Instant>>>,
    interval: Duration,
}

impl Database {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database {
            db: Arc::new(RwLock::new(Data::empty())),
            path: path.into(),
            last_call: Arc::new(RwLock::new(None)),
            interval: Duration::from_secs(5),
        }
    }

    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut Data),
    {
        f(&mut self.db.write().unwrap())
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Data) -> R,
    {
        f(&self.db.read().unwrap())
    }

    // pub fn refresh(&self) {
    //     let now = Instant::now();
    //     let mut last_call = self.last_call.lock().unwrap();

    //     let elapsed = now.duration_since(last_call.unwrap_or(now));
    //     *last_call = Some(now);

    //     if elapsed >= self.interval {
    //         self.force_refresh();
    //     }
    // }

    pub fn force_refresh(&self) {
        println!("[info] Refreshing content...");

        let new_data = load_data(&self.path);

        self.modify(move |data| {
            let version = data.version;
            *data = new_data;
            data.version = version + 1;
        });
    }
}
