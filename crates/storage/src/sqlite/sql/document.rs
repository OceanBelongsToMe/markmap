pub const LIST_BY_FOLDER: &str = r#"
    SELECT id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext
    FROM documents
    WHERE folder_id = ?
    ORDER BY path DESC
    "#;

pub const GET: &str = r#"
    SELECT id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext
    FROM documents
    WHERE id = ?
    "#;

pub const UPSERT: &str = r#"
    INSERT INTO documents (id, folder_id, path, title, content_hash, lang, updated_at, tree_id, ext)
    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ON CONFLICT(id) DO UPDATE SET
        folder_id = excluded.folder_id,
        path = excluded.path,
        title = excluded.title,
        content_hash = excluded.content_hash,
        lang = excluded.lang,
        updated_at = excluded.updated_at,
        tree_id = excluded.tree_id,
        ext = excluded.ext
    "#;

pub const DELETE: &str = "DELETE FROM documents WHERE id = ?";
