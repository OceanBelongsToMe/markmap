#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkmapLoadMode {
    Full,
    Lazy,
    Outline,
}

impl Default for MarkmapLoadMode {
    fn default() -> Self {
        Self::Outline
    }
}

#[derive(Debug, Clone)]
pub struct MarkmapOptions {
    pub initial_expand_level: i32,
    pub load_mode_root: MarkmapLoadMode,
    pub load_mode_child: MarkmapLoadMode,
}

impl Default for MarkmapOptions {
    fn default() -> Self {
        Self {
            initial_expand_level: -1,
            load_mode_root: MarkmapLoadMode::Outline,
            load_mode_child: MarkmapLoadMode::Lazy,
        }
    }
}
