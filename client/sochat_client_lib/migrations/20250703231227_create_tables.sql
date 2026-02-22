-- note: NOT NULL is not necessary for primary keys, however, sqlx doesn't know that
-- and deduces ids as Option<i64>. For convenience we just explicitly tell it that
-- primary keys can't be null
CREATE TABLE IF NOT EXISTS Hubs (
    url TEXT PRIMARY KEY NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS Accounts (
    id INTEGER PRIMARY KEY NOT NULL,
    hub_url TEXT NOT NULL REFERENCES Hubs(url) ON DELETE CASCADE,
    user_id BLOB NOT NULL,

    UNIQUE (hub_url, user_id)
) STRICT;

CREATE TABLE IF NOT EXISTS Chats (
    id INTEGER PRIMARY KEY NOT NULL,
    account_id INTEGER NOT NULL REFERENCES Accounts(id) ON DELETE CASCADE,
    other_id BLOB NOT NULL,

    UNIQUE (account_id, other_id)
) STRICT;

CREATE TABLE IF NOT EXISTS Messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,   -- internal client-side message ID
    hub_assigned_id BLOB DEFAULT NULL,
    chat_id INTEGER NOT NULL REFERENCES Chats(id) ON DELETE CASCADE,
    is_incoming INTEGER NOT NULL CHECK (
        is_incoming = FALSE
        OR (is_incoming = TRUE AND hub_assigned_id IS NOT NULL)
    ),
    content TEXT NOT NULL,

    UNIQUE(hub_assigned_id, chat_id)
) STRICT;

CREATE INDEX IF NOT EXISTS IdxMessagesChat On Messages (chat_id, id);
