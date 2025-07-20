use common::types::UserId;

use crate::account::Account;

#[derive(Debug, Clone)]
pub struct Chat {
    account: Account,
    other_id: UserId,
}

impl Chat {
    /// Send a single message to the other user
    pub fn send_message() {
        todo!()
    }

    /// Get a stream of incoming messages from the other user
    pub fn recv_messages() {
        todo!()
    }
}