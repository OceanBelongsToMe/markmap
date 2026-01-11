pub const GET: &str = r#"
    SELECT id, user_id, scope, scope_id, namespace, key, value_json, updated_at
    FROM user_settings
    WHERE user_id IS ?
      AND scope = ?
      AND scope_id IS ?
      AND namespace = ?
      AND key = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO user_settings (id, user_id, scope, scope_id, namespace, key, value_json, updated_at)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    ON CONFLICT(user_id, scope, scope_id, namespace, key) DO UPDATE SET
        id = excluded.id,
        value_json = excluded.value_json,
        updated_at = excluded.updated_at
    "#;
