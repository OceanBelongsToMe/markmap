pub const LIST_BY_WORKSPACE: &str = r#"
    SELECT workspace_id, document_id, last_opened_at, position
    FROM workspace_recent_files
    WHERE workspace_id = ?
    ORDER BY position ASC
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO workspace_recent_files (workspace_id, document_id, last_opened_at, position)
    VALUES (?, ?, ?, ?)
    ON CONFLICT(workspace_id, document_id) DO UPDATE SET
        last_opened_at = excluded.last_opened_at,
        position = excluded.position
    "#;

pub const DELETE: &str = r#"
    DELETE FROM workspace_recent_files
    WHERE workspace_id = ? AND document_id = ?
    "#;

pub const CLEAR_BY_WORKSPACE: &str = r#"
    DELETE FROM workspace_recent_files
    WHERE workspace_id = ?
    "#;
