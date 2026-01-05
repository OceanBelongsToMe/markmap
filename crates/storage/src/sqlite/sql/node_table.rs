pub const LIST_BY_DOC: &str = r#"
    SELECT t.node_id, t.align_json
    FROM node_table t
    INNER JOIN nodes n ON n.id = t.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, align_json
    FROM node_table
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_table (node_id, align_json)
    VALUES (?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        align_json = excluded.align_json
    "#;

pub const DELETE: &str = "DELETE FROM node_table WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_table
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
