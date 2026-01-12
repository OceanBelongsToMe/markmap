use common::types::AppResult;

use super::super::classify::classifier::NodeTypeClassifier;
use super::super::types::NodeTree;
use super::policy::SpacingPolicy;
use super::profile::MarkdownStyleProfile;

pub struct RenderEngine<'a> {
    pub(crate) tree: &'a NodeTree,
    pub(crate) classifier: &'a NodeTypeClassifier,
    pub(crate) profile: &'a MarkdownStyleProfile,
    pub(crate) spacing: &'a dyn SpacingPolicy,
}

impl<'a> RenderEngine<'a> {
    pub fn new(
        tree: &'a NodeTree,
        classifier: &'a NodeTypeClassifier,
        profile: &'a MarkdownStyleProfile,
        spacing: &'a dyn SpacingPolicy,
    ) -> Self {
        Self {
            tree,
            classifier,
            profile,
            spacing,
        }
    }

    pub fn render(&self) -> AppResult<String> {
        let mut lines = Vec::new();
        for node_id in &self.tree.roots {
            self.render_node(*node_id, None, "", 0, &mut lines);
        }
        self.spacing.trim_trailing_blank_lines(&mut lines);
        Ok(lines.join("\n"))
    }
}
