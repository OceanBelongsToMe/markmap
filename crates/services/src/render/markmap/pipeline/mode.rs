use crate::render::markmap::config::options::MarkmapLoadMode;
use crate::render::markmap::pipeline::{lazy, outline};
use crate::render::markmap::types::MarkmapNode;

pub fn apply_load_mode_root(root: &mut MarkmapNode, mode: MarkmapLoadMode) {
    match mode {
        MarkmapLoadMode::Outline => outline::apply_outline_root(root),
        MarkmapLoadMode::Full | MarkmapLoadMode::Lazy => {
            lazy::apply_load_mode_root(root, mode)
        }
    }
}
