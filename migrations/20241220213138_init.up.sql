CREATE TABLE users (
    username TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    pwd TEXT NOT NULL
);

CREATE TABLE moods (
    username TEXT NOT NULL PRIMARY KEY,
    is_active BOOLEAN NOT NULL,
    mood TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);

CREATE TABLE api_tokens (
    username TEXT NOT NULL PRIMARY KEY,
    token TEXT NOT NULL,
    created_at TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    can_change_pwd BOOLEAN NOT NULL,
    can_set_mood BOOLEAN NOT NULL,
    can_delete_user BOOLEAN NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);