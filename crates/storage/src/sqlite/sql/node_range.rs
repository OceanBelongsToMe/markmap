pub const LIST_BY_DOC: &str = r#"
    SELECT r.node_id, r.range_start, r.range_end, r.updated_at
    FROM node_range r
    INNER JOIN nodes n ON n.id = r.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, range_start, range_end, updated_at
    FROM node_range
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_range (node_id, range_start, range_end, updated_at)
    VALUES (?, ?, ?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        range_start = excluded.range_start,
        range_end = excluded.range_end,
        updated_at = excluded.updated_at
    "#;

pub const DELETE: &str = "DELETE FROM node_range WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_range
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
