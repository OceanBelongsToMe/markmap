pub const LIST_BY_WORKSPACE: &str = r#"
    SELECT id, workspace_id, root_path, created_at, updated_at
    FROM folders
    WHERE workspace_id = ?
    ORDER BY root_path DESC
    "#;

pub const GET: &str = r#"
    SELECT id, workspace_id, root_path, created_at, updated_at
    FROM folders
    WHERE id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO folders (id, workspace_id, root_path, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
        workspace_id = excluded.workspace_id,
        root_path = excluded.root_path,
        created_at = excluded.created_at,
        updated_at = excluded.updated_at
    "#;

pub const DELETE: &str = "DELETE FROM folders WHERE id = ?";
