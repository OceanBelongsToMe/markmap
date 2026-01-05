pub const LIST: &str = r#"
    SELECT id, name, config_profile_id, config_override_json, created_at, updated_at
    FROM workspaces
    ORDER BY created_at ASC
    "#;

pub const GET: &str = r#"
    SELECT id, name, config_profile_id, config_override_json, created_at, updated_at
    FROM workspaces
    WHERE id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO workspaces (id, name, config_profile_id, config_override_json, created_at, updated_at)
    VALUES (?, ?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
        name = excluded.name,
        config_profile_id = excluded.config_profile_id,
        config_override_json = excluded.config_override_json,
        created_at = excluded.created_at,
        updated_at = excluded.updated_at
    "#;

pub const DELETE: &str = "DELETE FROM workspaces WHERE id = ?";
