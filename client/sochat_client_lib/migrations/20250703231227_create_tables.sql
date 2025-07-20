-- note: NOT NULL is not necessary for primary keys, however, sqlx doesn't know that
-- and deduces ids as Option<i64>. For convenience we just explicitly tell it that
-- primary keys can't be null
CREATE TABLE IF NOT EXISTS Hubs (
    url TEXT PRIMARY KEY NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS Accounts (
    id INTEGER PRIMARY KEY NOT NULL,
    hub_url TEXT NOT NULL,
    user_id BLOB NOT NULL,

    UNIQUE (hub_url, user_id),
    FOREIGN KEY (hub_url) REFERENCES Hubs(url) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS Chats (
    id INTEGER PRIMARY KEY NOT NULL,
    account_id INTEGER NOT NULL,
    other_id BLOB NOT NULL,

    UNIQUE (account_id, other_id),
    FOREIGN KEY (account_id) REFERENCES Accounts(id) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS Messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,   -- internal client-side message ID
    hub_assigned_id BLOB DEFAULT NULL,
    chat_id INTEGER NOT NULL,
    is_incoming INTEGER NOT NULL CHECK (
        is_incoming = FALSE
        OR (is_incoming = TRUE AND hub_assigned_id IS NOT NULL)
    ),
    content TEXT NOT NULL,

    UNIQUE(hub_assigned_id, chat_id),
    FOREIGN KEY (chat_id) REFERENCES Chats(id) ON DELETE CASCADE
) STRICT;

CREATE INDEX IF NOT EXISTS IdxMessagesChat On Messages (chat_id, id);
