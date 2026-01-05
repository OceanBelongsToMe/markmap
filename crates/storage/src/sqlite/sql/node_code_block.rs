pub const LIST_BY_DOC: &str = r#"
    SELECT cb.node_id, cb.language
    FROM node_code_block cb
    INNER JOIN nodes n ON n.id = cb.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, language
    FROM node_code_block
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_code_block (node_id, language)
    VALUES (?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        language = excluded.language
    "#;

pub const DELETE: &str = "DELETE FROM node_code_block WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_code_block
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
