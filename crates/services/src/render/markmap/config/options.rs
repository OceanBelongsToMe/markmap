#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkmapLoadMode {
    Full,
    Lazy,
}

impl Default for MarkmapLoadMode {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Debug, Clone)]
pub struct MarkmapOptions {
    pub initial_expand_level: i32,
    pub load_mode: MarkmapLoadMode,
}

impl Default for MarkmapOptions {
    fn default() -> Self {
        Self {
            initial_expand_level: -1,
            load_mode: MarkmapLoadMode::default(),
        }
    }
}
