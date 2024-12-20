CREATE TABLE users (
    username TEXT NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    pwd TEXT NOT NULL
);

CREATE TABLE moods (
    id TEXT PRIMARY KEY,
    mood TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE api_tokens (
    token TEXT NOT NULL PRIMARY KEY,
    created_at TEXT NOT NULL,
    is_active BOOLEAN NOT NULL,
    can_change_pwd BOOLEAN NOT NULL,
    can_set_mood BOOLEAN NOT NULL,
    can_delete_user BOOLEAN NOT NULL
);

CREATE TABLE users_moods (
    user_username TEXT NOT NULL REFERENCES users (username) ON DELETE CASCADE,
    mood_id TEXT not null REFERENCES moods (id) ON DELETE CASCADE,
    PRIMARY KEY (user_username, mood_id)
);

CREATE TABLE users_api_tokens (
    user_username TEXT NOT NULL REFERENCES users (username) ON DELETE CASCADE,
    api_token_token TEXT NOT NULL REFERENCES api_tokens (token) ON DELETE CASCADE,
    PRIMARY KEY (user_username, api_token_token)
);