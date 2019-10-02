use crate::prelude::*;

use std::sync::{Arc, Mutex};

use crate::load::load_data;

pub struct Database {
    db: Arc<Mutex<Data>>,
    path: PathBuf,
}

impl Database {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Database {
            db: Arc::new(Mutex::new(Data::empty())),
            path: path.into(),
        }
    }

    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut Data),
    {
        f(&mut self.db.lock().unwrap())
    }

    pub fn read<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Data) -> R,
    {
        f(&self.db.lock().unwrap())
    }

    pub fn refresh(&self) {
        let new_data = load_data(&self.path).unwrap();

        self.modify(move |data| {
            let version = data.version;
            *data = new_data;
            data.version = version + 1;
        });
    }
}
