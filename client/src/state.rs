use std::sync::Arc;

use tokio::{
    runtime::Handle,
    sync::{Mutex as TokioMutex, MutexGuard as TokioMutexGuard},
};

#[derive(Debug, Clone)]
struct InnerAppState {
    messages: Vec<String>,
}

// TODO: look into ways of making it impossible to forget to request repaint from egui

/// A lock guard that provides access to the shared application state
#[derive(Debug)]
pub struct AppStateLock<'a> {
    guard: TokioMutexGuard<'a, InnerAppState>,
    // this is so that we can request redraws here when stuff happens
    egui_ctx: egui::Context,
}

impl<'a> AppStateLock<'a> {
    pub fn insert_message(&mut self, message: String) {
        self.guard.messages.push(message);
        self.egui_ctx.request_repaint();
    }

    pub fn messages(&self) -> impl Iterator<Item = &str> {
        self.guard.messages.iter().map(|m| m.as_str())
    }
}

/// Central store for the state of the app
///
/// Is actually a wrapper around the reference counted shared state,
/// therefore, cloning is cheap.
///
/// Most actions with the state require locking it first
#[derive(Debug, Clone)]
pub struct AppState {
    inner: Arc<TokioMutex<InnerAppState>>,
    tokio_handle: Handle,
    egui_ctx: egui::Context,
}

impl AppState {
    pub fn new(tokio_handle: Handle, egui_ctx: egui::Context) -> Self {
        Self {
            inner: Arc::new(TokioMutex::new(InnerAppState {
                messages: Vec::new(),
            })),
            tokio_handle,
            egui_ctx,
        }
    }

    pub fn factory(
        tokio_handle: Handle,
    ) -> impl (FnOnce(&eframe::CreationContext) -> Self) + 'static {
        move |cc| Self::new(tokio_handle, cc.egui_ctx.clone())
    }

    pub fn lock_blocking(&self) -> AppStateLock<'_> {
        AppStateLock {
            guard: self.inner.blocking_lock(),
            egui_ctx: self.egui_ctx.clone(),
        }
    }

    pub async fn lock(&self) -> AppStateLock<'_> {
        AppStateLock {
            guard: self.inner.lock().await,
            egui_ctx: self.egui_ctx.clone(),
        }
    }

    pub fn async_handle(&self) -> &Handle {
        &self.tokio_handle
    }

    pub fn egui_ctx(&self) -> &egui::Context {
        &self.egui_ctx
    }
}
