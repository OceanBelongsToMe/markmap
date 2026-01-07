pub const LIST_BY_DOC: &str = r#"
SELECT node_footnote_definition.node_id, node_footnote_definition.label
FROM node_footnote_definition
JOIN nodes ON nodes.id = node_footnote_definition.node_id
WHERE nodes.doc_id = ?
ORDER BY node_footnote_definition.node_id
"#;

pub const GET: &str = r#"
SELECT node_id, label
FROM node_footnote_definition
WHERE node_id = ?
"#;

pub const UPSERT: &str = r#"
INSERT INTO node_footnote_definition (node_id, label)
VALUES (?, ?)
ON CONFLICT(node_id) DO UPDATE SET
    label = excluded.label
"#;

pub const DELETE: &str = r#"
DELETE FROM node_footnote_definition
WHERE node_id = ?
"#;

pub const DELETE_BY_DOC: &str = r#"
DELETE FROM node_footnote_definition
WHERE node_id IN (
    SELECT id FROM nodes WHERE doc_id = ?
)
"#;
