use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};

use client_lib::common::types::{Id, UserId};
use flutter_rust_bridge::frb;

#[frb(ignore)]
#[derive(Debug, Default)]
struct ServiceInner {
    messages: HashMap<(UserId, UserId), Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Service {
    this: UserId,
    messages: Arc<Mutex<ServiceInner>>,
}

impl Service {
    #[frb(sync)]
    pub fn new() -> Self {
        Self {
            this: UserId::generate(),
            messages: Arc::default(),
        }
    }

    fn inner(&self) -> impl DerefMut<Target = ServiceInner> + '_ {
        self.messages.lock().unwrap()
    }
    
    // IMPORTANT!!!
    // MAKE SURE TO TAKE IDs by reference,
    // This is because if you take them by value
    // FRB will move them out of Arcs and dispose of the Arcs
    // Causing "Arc" used after being disposed errors
    
    #[frb(sync)]
    pub fn message_count(&self, from: &UserId, to: &UserId) -> i64 {
        self.inner()
            .messages
            .get(&(*from, *to))
            .map(|v| v.len())
            .unwrap_or(0) as _
    }

    #[frb(sync)]
    pub fn get_message(&self, from: &UserId, to: &UserId, index: i64) -> Option<String> {
        self.inner()
            .messages
            .get(&(*from, *to))?
            .get(usize::try_from(index).unwrap())
            .cloned()
    }

    #[frb(sync)]
    pub fn send_message(&self, from: &UserId, to: &UserId, message: String) {
        self.inner()
            .messages
            .entry((*from, *to))
            .or_default()
            .push(message);
    }
}
