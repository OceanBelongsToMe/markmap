use crate::render::markmap::config::options::MarkmapOptions;
use crate::render::markmap::traits::MarkmapFolding;
use crate::render::markmap::types::MarkmapNode;

pub struct FoldPolicy;

impl FoldPolicy {
    fn apply_node(node: &mut MarkmapNode, fold_recursively: &mut u32, options: &MarkmapOptions) {
        let is_fold_recursively = node.payload.fold == Some(2);
        if is_fold_recursively {
            *fold_recursively += 1;
        } else if *fold_recursively > 0
            || (options.initial_expand_level >= 0
                && node.state.depth as i32 >= options.initial_expand_level)
        {
            node.payload.fold = Some(1);
        }
        node.payload.update_children_indicator();

        for child in node.children.iter_mut() {
            Self::apply_node(child, fold_recursively, options);
        }

        if is_fold_recursively {
            *fold_recursively -= 1;
        }
    }
}

impl MarkmapFolding for FoldPolicy {
    fn apply(&self, root: &mut MarkmapNode, options: &MarkmapOptions) {
        let mut fold_recursively = 0u32;
        Self::apply_node(root, &mut fold_recursively, options);
    }
}
