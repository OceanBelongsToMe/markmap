use knowlattice_core::model::NodeId;

use super::engine::RenderEngine;
use super::rules::{block_prefix, table_separator_with_alignment};
use super::super::classify::classifier::MarkdownKind;

impl RenderEngine<'_> {
    pub fn table_row_cells(&self, node_id: NodeId) -> Vec<String> {
        let mut cells = Vec::new();
        for child_id in self.children(node_id) {
            let Some(child) = self.tree.nodes_by_id.get(&child_id) else {
                continue;
            };
            let kind = self.classifier.classify(child.base.node_type_id);
            if kind == MarkdownKind::TableCell {
                let text = self.collect_inline(child_id);
                cells.push(text);
            }
        }
        if cells.is_empty() {
            let text = self.collect_inline(node_id);
            if !text.trim().is_empty() {
                cells.push(text);
            }
        }
        cells
    }

    pub fn render_table(
        &self,
        node_id: NodeId,
        indent: &str,
        quote_depth: usize,
        out: &mut Vec<String>,
    ) {
        let Some(record) = self.tree.nodes_by_id.get(&node_id) else {
            return;
        };
        let alignments = record
            .table
            .as_ref()
            .map(|table| table.alignments.clone())
            .unwrap_or_default();
        let mut head_rows = Vec::new();
        let mut body_rows = Vec::new();
        for child_id in self.children(node_id) {
            let Some(child) = self.tree.nodes_by_id.get(&child_id) else {
                continue;
            };
            let kind = self.classifier.classify(child.base.node_type_id);
            match kind {
                MarkdownKind::TableHead => {
                    for row_id in self.children(child_id) {
                        head_rows.push(self.table_row_cells(row_id));
                    }
                }
                MarkdownKind::TableRow => {
                    body_rows.push(self.table_row_cells(child_id));
                }
                _ => {}
            }
        }
        let prefix = block_prefix(indent, quote_depth);
        if let Some(head) = head_rows.first() {
            let count = head.len().max(alignments.len());
            if !head.is_empty() {
                out.push(format!("{prefix}| {} |", head.join(" | ")));
                out.push(format!(
                    "{prefix}|{}|",
                    table_separator_with_alignment(count, &alignments)
                ));
            }
        } else if !body_rows.is_empty() {
            let count = body_rows[0].len().max(alignments.len());
            out.push(format!(
                "{prefix}|{}|",
                table_separator_with_alignment(count, &alignments)
            ));
        }
        for row in &body_rows {
            if row.is_empty() {
                continue;
            }
            out.push(format!("{prefix}| {} |", row.join(" | ")));
        }
        if head_rows.is_empty() && body_rows.is_empty() {
            for child_id in self.children(node_id) {
                self.render_node(child_id, None, indent, quote_depth, out);
            }
        }
    }
}
