use std::sync::{Arc, Mutex, MutexGuard};

use common::types::UserId;

/// Underlying data of the store
#[derive(Debug, Clone)]
pub struct InnerStore {
    self_id: UserId,
    other_id: Option<UserId>,
    messages: Vec<String>,
    message_box_text: String,
    other_id_input_line: String,
}

/// A lock guard that provides access to the shared store
#[derive(Debug)]
pub struct StoreLock<'a> {
    guard: MutexGuard<'a, InnerStore>,
}

impl<'a> StoreLock<'a> {
    pub fn messages(&self) -> impl Iterator<Item = &str> {
        self.guard.messages.iter().map(|m| m.as_str())
    }
    
    pub fn insert_message(&mut self, message: String) {
        self.guard.messages.push(message);
    }
    
    pub fn message_box_text(&mut self) -> &mut String {
        &mut self.guard.message_box_text
    }
    
    pub fn other_id_input_line(&mut self) -> &mut String {
        &mut self.guard.other_id_input_line
    }
    
    pub fn self_id(&self) -> UserId {
        self.guard.self_id
    }
    
    pub fn other_id(&self) -> Option<UserId> {
        self.guard.other_id
    }
    
    pub fn set_other_id(&mut self, value: UserId) {
        self.guard.other_id = Some(value);
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
    inner: Arc<Mutex<InnerStore>>,
}

impl Store {
    pub fn new(self_id: UserId) -> Self {
        Self {
            inner: Arc::new(Mutex::new(InnerStore {
                messages: Vec::new(),
                message_box_text: String::new(),
                other_id_input_line: String::new(),      
                self_id,  
                other_id: None
            })),
        }
    }

    pub fn lock(&self) -> StoreLock<'_> {
        StoreLock {
            guard: self.inner.lock().expect("Mutex poisoned"),
        }
    }
}
