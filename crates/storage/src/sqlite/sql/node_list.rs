pub const LIST_BY_DOC: &str = r#"
    SELECT l.node_id, l.ordering, l.is_item
    FROM node_list l
    INNER JOIN nodes n ON n.id = l.node_id
    WHERE n.doc_id = ?
    ORDER BY n.created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT node_id, ordering, is_item
    FROM node_list
    WHERE node_id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO node_list (node_id, ordering, is_item)
    VALUES (?, ?, ?)
    ON CONFLICT(node_id) DO UPDATE SET
        ordering = excluded.ordering,
        is_item = excluded.is_item
    "#;

pub const DELETE: &str = "DELETE FROM node_list WHERE node_id = ?";

pub const DELETE_BY_DOC: &str = r#"
    DELETE FROM node_list
    WHERE node_id IN (
        SELECT id
        FROM nodes
        WHERE doc_id = ?
    )
    "#;
