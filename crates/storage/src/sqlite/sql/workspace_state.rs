pub const GET: &str = r#"
    SELECT current_workspace_id, updated_at
    FROM workspace_state
    WHERE id = 1
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO workspace_state (id, current_workspace_id, updated_at)
    VALUES (1, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
        current_workspace_id = excluded.current_workspace_id,
        updated_at = excluded.updated_at
    "#;
