pub const LIST_BY_DOC: &str = r#"
    SELECT id, doc_id, parent_id, node_type_id, created_at, updated_at
    FROM nodes
    WHERE doc_id = ?
    ORDER BY created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT id, doc_id, parent_id, node_type_id, created_at, updated_at
    FROM nodes
    WHERE id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO nodes (id, doc_id, parent_id, node_type_id, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
        doc_id = excluded.doc_id,
        parent_id = excluded.parent_id,
        node_type_id = excluded.node_type_id,
        created_at = excluded.created_at,
        updated_at = excluded.updated_at
    "#;

pub const DELETE_BY_DOC: &str = "DELETE FROM nodes WHERE doc_id = ?";
