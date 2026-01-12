#[derive(Debug, Clone)]
pub struct MarkmapOptions {
    pub initial_expand_level: i32,
}

impl Default for MarkmapOptions {
    fn default() -> Self {
        Self {
            initial_expand_level: -1,
        }
    }
}
