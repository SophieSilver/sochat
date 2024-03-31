use std::sync::Arc;

use tokio::sync::{Mutex as TokioMutex, MutexGuard as TokioMutexGuard};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct InnerAppState {}

#[derive(Debug)]
pub struct AppStateGuard<'a> {
    guard: TokioMutexGuard<'a, InnerAppState>,
}

impl<'a> AppStateGuard<'a> {}

#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<TokioMutex<InnerAppState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(TokioMutex::new(InnerAppState {})),
        }
    }

    pub fn lock_blocking(&self) -> AppStateGuard<'_> {
        AppStateGuard {
            guard: self.inner.blocking_lock(),
        }
    }

    pub async fn lock(&self) -> AppStateGuard<'_> {
        AppStateGuard {
            guard: self.inner.lock().await,
        }
    }
}
