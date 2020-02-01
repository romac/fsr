use crate::db::Database;
use crate::DB;

#[derive(Clone, Copy)]
pub struct Db;

impl AsRef<Database> for Db {
    fn as_ref(&self) -> &Database {
        &DB
    }
}
