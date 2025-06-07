//! Module for managing global state of the server

use sqlx::SqlitePool;

use crate::db::Db;

// #[derive(Debug)]
// struct UncloneableState {
    
// }

/// The state of the entire application
#[derive(Debug, Clone)]
pub struct AppState {
    // everything inside of a state must be cheaply cloneable, e.g. be an Arc around some other struct
    // pool in an Arc under the hood, no need to wrap that
    db_connection_pool: SqlitePool,
}

impl AppState {
    /// Create a new instance of the state
    pub fn new(db_connection_pool: SqlitePool) -> Self {
        Self {
            db_connection_pool,
        }
    }
    
    /// Get a reference to the Database connection pool
    pub fn db(&self) -> &(impl Db + Send + Sync) {
        &self.db_connection_pool
    }
}
