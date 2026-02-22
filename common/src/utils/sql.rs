/// Convenience methods for analyzing [`sqlx::Error`]
pub trait SqlxErrorExt {
    /// Check if the error is a `UNIQUE` contraint violation
    fn is_unique_violation(&self) -> bool;
}

impl SqlxErrorExt for sqlx::Error {
    fn is_unique_violation(&self) -> bool {
        self.as_database_error()
            .is_some_and(|e| e.is_unique_violation())
    }
}
