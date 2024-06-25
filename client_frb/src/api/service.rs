use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use client_service::common::types::UserId;
use flutter_rust_bridge::frb;

#[derive(Debug, Default)]
struct ServiceInner {
    messages: HashMap<(UserId, UserId), Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Service {
    messages: Arc<Mutex<ServiceInner>>,
}

impl Service {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            messages: Arc::default(),
        }
    }

    fn inner(&self) -> impl DerefMut<Target = ServiceInner> + '_ {
        self.messages.lock().unwrap()
    }

    #[frb(sync)]
    pub fn message_count(&self, from: UserId, to: UserId) -> i64 {
        self.inner()
            .messages
            .get(&(from, to))
            .map(|v| v.len())
            .unwrap_or(0) as _
    }

    #[frb(sync)]
    pub fn get_message(&self, from: UserId, to: UserId, index: i64) -> Option<String> {
        self.inner()
            .messages
            .get(&(from, to))?
            .get(usize::try_from(index).unwrap())
            .cloned()
    }

    #[frb(sync)]
    pub fn send_message(&self, from: UserId, to: UserId, message: String) {
        self.inner()
            .messages
            .entry((from, to))
            .or_default()
            .push(message);
    }
}
