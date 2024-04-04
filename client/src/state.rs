use std::future::Future;

use crate::gui::store::Store;
use eframe::CreationContext;
use egui::Context;
use tokio::runtime::Handle;

pub mod repaint_store_lock {
    use std::ops::{Deref, DerefMut};

    use crate::gui::store::StoreLock;

    /// A wrapper for [`StoreLockMut`] that will automatically request repaint from egui after being dropped
    #[derive(Debug)]
    pub struct RepaintStoreLock<'a> {
        store_lock: StoreLock<'a>,
        egui_ctx: egui::Context,
    }

    impl<'a> RepaintStoreLock<'a> {
        pub fn new(store_lock: StoreLock<'a>, egui_ctx: egui::Context) -> Self {
            Self {
                store_lock,
                egui_ctx,
            }
        }
    }

    impl<'a> Deref for RepaintStoreLock<'a> {
        type Target = StoreLock<'a>;

        fn deref(&self) -> &Self::Target {
            &self.store_lock
        }
    }

    impl<'a> DerefMut for RepaintStoreLock<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.store_lock
        }
    }

    impl<'a> Drop for RepaintStoreLock<'a> {
        fn drop(&mut self) {
            self.egui_ctx.request_repaint();
        }
    }
}

pub use repaint_store_lock::RepaintStoreLock;

#[derive(Debug, Clone)]
pub struct AppState {
    pub ui_store: Store,
    pub tokio_handle: Handle,
    pub egui_ctx: Context,
}

impl AppState {
    pub fn new(ui_store: Store, tokio_handle: Handle, egui_ctx: Context) -> Self {
        Self {
            ui_store,
            tokio_handle,
            egui_ctx,
        }
    }

    /// Get a factory function that will construct the [`AppState`] from a [`CreationContext`]
    pub fn factory(
        ui_store: Store,
        tokio_handle: Handle,
    ) -> impl (FnOnce(&CreationContext) -> Self) + 'static {
        move |cc| Self::new(ui_store, tokio_handle, cc.egui_ctx.clone())
    }

    /// Get a factory function that will construct the [`AppState`] from a [`CreationContext`]
    /// and call the callback function with the constructed [`AppState`].
    ///
    /// This can be used, to, for example, start up tasks that require the [`AppState`]
    pub fn factory_with_callback<F>(
        ui_store: Store,
        tokio_handle: Handle,
        callback: F,
    ) -> impl (FnOnce(&CreationContext) -> Self) + 'static
    where
        F: FnOnce(&Self) + 'static,
    {
        let factory = Self::factory(ui_store, tokio_handle);

        |cc| {
            let app_state = factory(cc);
            callback(&app_state);
            app_state
        }
    }

    /// Helper function to spawn async tasks from synchronouse contexts
    ///
    /// equivalent to `app_state.`
    pub fn run_async<F>(&self, future: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.tokio_handle.spawn(future);
    }

    /// Blocking variant of `lock_store_with_repaint`
    pub fn lock_store_with_repaint_blocking(&self) -> RepaintStoreLock {
        RepaintStoreLock::new(self.ui_store.lock_blocking(), self.egui_ctx.clone())
    }

    /// Mutably lock the store and wrap it in [`RepaintStoreLock`],
    /// which will request repaint from [`egui`] upon falling out of scope
    pub async fn lock_store_with_repaint(&self) -> RepaintStoreLock {
        RepaintStoreLock::new(self.ui_store.lock().await, self.egui_ctx.clone())
    }
}
