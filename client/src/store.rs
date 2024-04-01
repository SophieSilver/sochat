use std::sync::Arc;

use tokio::sync::{Mutex as TokioMutex, MutexGuard as TokioMutexGuard};

/// Underlying data of the store
#[derive(Debug, Clone)]
pub struct InnerStore {
    messages: Vec<String>,
}

/// A lock guard that provides immutable access to the shared store
#[derive(Debug)]
pub struct StoreLock<'a> {
    guard: TokioMutexGuard<'a, InnerStore>,
}

impl<'a> StoreLock<'a> {
    pub fn messages(&self) -> impl Iterator<Item = &str> {
        self.guard.messages.iter().map(|m| m.as_str())
    }
}

/// a lock guard that provides mutable access to the shared store
#[derive(Debug)]
pub struct StoreLockMut<'a> {
    guard: TokioMutexGuard<'a, InnerStore>,
}

impl<'a> StoreLockMut<'a> {
    pub fn insert_message(&mut self, message: String) {
        self.guard.messages.push(message);
    }
}

/// Central store for the state of the GUI
///
/// Is actually a wrapper around the reference counted shared state,
/// therefore, cloning is cheap.
///
/// Most actions require first acquiring a lock around the state
#[derive(Debug, Clone)]
pub struct Store {
    inner: Arc<TokioMutex<InnerStore>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(TokioMutex::new(InnerStore {
                messages: Vec::new(),
            })),
        }
    }

    pub fn lock_blocking(&self) -> StoreLock<'_> {
        StoreLock {
            guard: self.inner.blocking_lock(),
        }
    }

    pub async fn lock(&self) -> StoreLock<'_> {
        StoreLock {
            guard: self.inner.lock().await,
        }
    }

    pub fn lock_blocking_mut(&self) -> StoreLockMut<'_> {
        StoreLockMut {
            guard: self.inner.blocking_lock(),
        }
    }

    pub async fn lock_mut(&self) -> StoreLockMut<'_> {
        StoreLockMut {
            guard: self.inner.lock().await,
        }
    }
}
