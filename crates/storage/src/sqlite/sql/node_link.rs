pub const LIST_BY_DOC: &str = r#"
    SELECT l.node_id, l.href, l.title, l.link_type, l.ref_id
    FROM node_link l
    INNER JOIN nodes n ON n.id = l.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, href, title, link_type, ref_id
    FROM node_link
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_link (node_id, href, title, link_type, ref_id)
    VALUES (?, ?, ?, ?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        href = excluded.href,
        title = excluded.title,
        link_type = excluded.link_type,
        ref_id = excluded.ref_id
    "#;

pub const DELETE: &str = "DELETE FROM node_link WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_link
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
