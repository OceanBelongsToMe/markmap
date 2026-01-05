pub const LIST_BY_DOC: &str = r#"
    SELECT w.node_id, w.target_node_id, w.display_text, w.created_at, w.updated_at
    FROM node_wiki w
    INNER JOIN nodes n ON n.id = w.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, target_node_id, display_text, created_at, updated_at
    FROM node_wiki
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_wiki (node_id, target_node_id, display_text, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        target_node_id = excluded.target_node_id,
        display_text = excluded.display_text,
        created_at = excluded.created_at,
        updated_at = excluded.updated_at
    "#;

pub const DELETE: &str = "DELETE FROM node_wiki WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_wiki
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
