use crate::chat::ChatId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub(crate) struct MessageId(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum MessageDirection {
    Outgoing = 0,
    Incoming = 1,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct MessageData {
    hub_assigned_id: MessageId,
    chat_id: ChatId,
    is_incoming: bool,
    /// On the SQL side this is called `is_incoming`
    direction: MessageDirection,
    content: String,
}

pub(crate) struct MessageRecord {
    id: 
}
