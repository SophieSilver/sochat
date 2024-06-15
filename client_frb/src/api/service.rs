use std::sync::{Arc, Mutex};

use flutter_rust_bridge::frb;

#[derive(Debug, Clone)]
pub struct Service {
    messages: Arc<Mutex<Vec<String>>>,
}

impl Service {
    #[frb(sync)]
    pub fn new() -> Self {
        Self { messages: Arc::default() }
    }
    
    #[frb(sync, type_64bit_int)]
    pub fn message_count(&self) -> usize {
        self.messages.lock().unwrap().len()
    }

    #[frb(sync, type_64bit_int)]
    pub fn get_message(&self, index: usize) -> Option<String> {
        self.messages.lock().unwrap().get(index).cloned()
    }

    #[frb(sync)]
    pub fn add_message(&self, message: String) {
        self.messages.lock().unwrap().push(message);
    }
}
