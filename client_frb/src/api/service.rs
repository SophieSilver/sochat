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
    
    #[frb(sync)]
    pub fn message_count(&self) -> i64 {
        self.messages.lock().unwrap().len() as i64
    }

    #[frb(sync)]
    pub fn get_message(&self, index: i64) -> Option<String> {
        self.messages.lock().unwrap().get(index as usize).cloned()
    }

    #[frb(sync)]
    pub fn add_message(&self, message: String) {
        self.messages.lock().unwrap().push(message);
    }
}
