CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY
) STRICT;

CREATE TABLE IF NOT EXISTS messages (
    id BLOB PRIMARY KEY,
    sender_id BLOB NOT NULL,
    recipient_id BLOB NOT NULL,
    content BLOB NOT NULL,
    is_received INTEGER NOT NULL DEFAULT FALSE CHECK (
        is_received = FALSE
        OR is_received = TRUE
    ),
    FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (recipient_id) REFERENCES users(id) ON DELETE CASCADE
) STRICT;

-- used when the recipient wants to fetch messages from a particular user (or from all users) 
-- that haven't yet been received
CREATE INDEX IF NOT EXISTS idx_messages_recieved_recipient_sender ON messages (is_received, recipient_id, sender_id);

-- -- when we want all messages between a particular set of users but don't care about receive status
-- CREATE INDEX IF NOT EXISTS idx_messages_recipient_sender ON messages(recipient_id, sender_id);
-- -- when we want all messages from a particular sender
-- CREATE INDEX IF NOT EXISTS idx_messages_sender ON messages(sender_id);

CREATE TABLE IF NOT EXISTS signing_keys (
    id BLOB PRIMARY KEY,
    signing_key BLOB NOT NULL UNIQUE,
    created INTEGER NOT NULL
) STRICT;

CREATE INDEX IF NOT EXISTS idx_signing_keys_created ON signing_keys(created);
