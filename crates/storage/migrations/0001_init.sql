PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS workspaces (
    id BLOB PRIMARY KEY,
    name TEXT NOT NULL,
    config_profile_id TEXT,
    config_override_json TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS folders (
    id BLOB PRIMARY KEY,
    workspace_id BLOB NOT NULL,
    root_path TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS documents (
    id BLOB PRIMARY KEY,
    folder_id BLOB NOT NULL,
    path TEXT NOT NULL,
    title TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    lang TEXT,
    updated_at INTEGER NOT NULL,
    tree_id TEXT,
    ext TEXT,
    FOREIGN KEY (folder_id) REFERENCES folders(id) ON DELETE CASCADE,
    UNIQUE (folder_id, path)
);

CREATE TABLE IF NOT EXISTS node_types (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

INSERT INTO node_types (id, name) VALUES
    (1, 'Heading'),
    (2, 'List'),
    (3, 'ListItem'),
    (4, 'CodeBlock'),
    (5, 'Table'),
    (6, 'Image'),
    (7, 'Link'),
    (8, 'Task'),
    (9, 'Wiki')
ON CONFLICT(id) DO NOTHING;

CREATE TABLE IF NOT EXISTS nodes (
    id BLOB PRIMARY KEY,
    doc_id BLOB NOT NULL,
    parent_id BLOB,
    node_type_id INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (doc_id) REFERENCES documents(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES nodes(id) ON DELETE SET NULL,
    FOREIGN KEY (node_type_id) REFERENCES node_types(id)
);

CREATE TABLE IF NOT EXISTS node_text (
    node_id BLOB PRIMARY KEY,
    text TEXT NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_range (
    node_id BLOB PRIMARY KEY,
    range_start INTEGER NOT NULL,
    range_end INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_heading (
    node_id BLOB PRIMARY KEY,
    level INTEGER NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_list (
    node_id BLOB PRIMARY KEY,
    ordering INTEGER NOT NULL,
    is_item INTEGER NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_code_block (
    node_id BLOB PRIMARY KEY,
    language TEXT,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_table (
    node_id BLOB PRIMARY KEY,
    align_json TEXT NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_image (
    node_id BLOB PRIMARY KEY,
    src TEXT NOT NULL,
    alt TEXT,
    title TEXT,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_link (
    node_id BLOB PRIMARY KEY,
    href TEXT NOT NULL,
    title TEXT,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_task (
    node_id BLOB PRIMARY KEY,
    checked INTEGER NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS node_wiki (
    node_id BLOB PRIMARY KEY,
    target_node_id BLOB NOT NULL,
    display_text TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE,
    FOREIGN KEY (target_node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_folders_workspace_id ON folders(workspace_id);
CREATE INDEX IF NOT EXISTS idx_documents_folder_id ON documents(folder_id);
CREATE INDEX IF NOT EXISTS idx_documents_path ON documents(path);
CREATE INDEX IF NOT EXISTS idx_nodes_doc_id ON nodes(doc_id);
CREATE INDEX IF NOT EXISTS idx_nodes_parent_id ON nodes(parent_id);
