pub const LIST_BY_DOC: &str = r#"
    SELECT i.node_id, i.src, i.alt, i.title
    FROM node_image i
    INNER JOIN nodes n ON n.id = i.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, src, alt, title
    FROM node_image
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_image (node_id, src, alt, title)
    VALUES (?, ?, ?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        src = excluded.src,
        alt = excluded.alt,
        title = excluded.title
    "#;

pub const DELETE: &str = "DELETE FROM node_image WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_image
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
