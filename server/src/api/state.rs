use sqlx::SqlitePool;

#[derive(Debug)]
struct UncloneableState {
    
}

#[derive(Debug, Clone)]
pub struct AppState {
    // everything inside of a state must be cheaply cloneable, e.g. be an Arc around some other struct
    // pool in an Arc under the hood, no need to wrap that
    db_connection_pool: SqlitePool,
}

impl AppState {
    pub fn new(db_connection_pool: SqlitePool) -> Self {
        Self {
            db_connection_pool,
        }
    }
    
    pub fn db_pool(&self) -> &SqlitePool {
        &self.db_connection_pool
    }
}
