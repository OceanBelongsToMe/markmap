CREATE TABLE IF NOT EXISTS node_table_new (
    node_id BLOB PRIMARY KEY,
    align_json TEXT NOT NULL,
    FOREIGN KEY (node_id) REFERENCES nodes(id) ON DELETE CASCADE
);

INSERT INTO node_table_new (node_id, align_json)
SELECT node_id, '[]'
FROM node_table;

DROP TABLE node_table;

ALTER TABLE node_table_new RENAME TO node_table;
