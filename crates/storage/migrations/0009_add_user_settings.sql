CREATE TABLE IF NOT EXISTS user_settings (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    scope TEXT NOT NULL,
    scope_id TEXT,
    namespace TEXT NOT NULL,
    key TEXT NOT NULL,
    value_json TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    UNIQUE (user_id, scope, scope_id, namespace, key)
);

CREATE INDEX IF NOT EXISTS user_settings_scope_idx
ON user_settings (namespace, scope, scope_id);
