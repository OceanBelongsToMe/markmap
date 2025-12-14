use serde_json::Value;

/// Pure Rust implementation that annotates each node's `payload.content_len`
/// when `content` exists. This crate is intended to be used directly from
/// a Tauri backend (Rust), via CLI, or as a library in server-side code.

fn walk_value(node: &mut Value) {
    if let Some(obj) = node.as_object_mut() {
        if let Some(content) = obj.get("content").and_then(|c| c.as_str()) {
            let len = content.len();
            match obj.get_mut("payload") {
                Some(p) if p.is_object() => {
                    if let Some(pobj) = p.as_object_mut() {
                        pobj.insert("content_len".to_string(), Value::from(len));
                    }
                }
                _ => {
                    obj.insert(
                        "payload".to_string(),
                        serde_json::json!({ "content_len": len }),
                    );
                }
            }
        }
        if let Some(children) = obj.get_mut("children").and_then(|c| c.as_array_mut()) {
            for child in children.iter_mut() {
                walk_value(child);
            }
        }
    }
}

/// Process a tree represented as `serde_json::Value`.
pub fn process_tree_value(mut tree: Value) -> Value {
    walk_value(&mut tree);
    tree
}

/// Process a tree encoded as JSON string; returns JSON string or error.
pub fn process_tree_json(input: &str) -> Result<String, serde_json::Error> {
    let mut v: Value = serde_json::from_str(input)?;
    walk_value(&mut v);
    serde_json::to_string(&v)
}

/// Version helper
pub fn version() -> &'static str {
    "markmap_common-0.1.0"
}
