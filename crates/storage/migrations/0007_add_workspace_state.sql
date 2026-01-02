PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS workspace_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    current_workspace_id BLOB NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (current_workspace_id) REFERENCES workspaces(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS workspace_recent_files (
    workspace_id BLOB NOT NULL,
    document_id BLOB NOT NULL,
    last_opened_at INTEGER NOT NULL,
    position INTEGER NOT NULL,
    PRIMARY KEY (workspace_id, document_id),
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE,
    FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_workspace_recent_files_workspace_id
    ON workspace_recent_files(workspace_id);
CREATE INDEX IF NOT EXISTS idx_workspace_recent_files_last_opened_at
    ON workspace_recent_files(last_opened_at);
