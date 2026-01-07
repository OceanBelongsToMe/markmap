CREATE TABLE IF NOT EXISTS node_footnote_definition (
    node_id BLOB PRIMARY KEY,
    label TEXT NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);
