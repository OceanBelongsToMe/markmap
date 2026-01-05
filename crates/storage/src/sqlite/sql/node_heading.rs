pub const LIST_BY_DOC: &str = r#"
    SELECT h.node_id, h.level
    FROM node_heading h
    INNER JOIN nodes n ON n.id = h.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, level
    FROM node_heading
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_heading (node_id, level)
    VALUES (?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        level = excluded.level
    "#;

pub const DELETE: &str = "DELETE FROM node_heading WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_heading
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
