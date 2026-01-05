pub const LIST: &str = r#"
    SELECT id, name
    FROM node_types
    ORDER BY id ASC
    "#;

pub const GET: &str = r#"
    SELECT id, name
    FROM node_types
    WHERE id = ?
    "#;
