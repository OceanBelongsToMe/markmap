use std::collections::HashMap;

use common::types::AppResult;
use knowlattice_core::model::NodeId;

use super::super::types::{NodeRecord, NodeSnapshot};

pub trait NodeAssembler: Send + Sync {
    fn assemble(&self, snapshot: NodeSnapshot) -> AppResult<HashMap<NodeId, NodeRecord>>;
}

pub struct DefaultAssembler;

impl DefaultAssembler {
    pub fn new() -> Self {
        Self
    }
}

impl NodeAssembler for DefaultAssembler {
    fn assemble(&self, snapshot: NodeSnapshot) -> AppResult<HashMap<NodeId, NodeRecord>> {
        let text_by_id = snapshot
            .texts
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let range_by_id = snapshot
            .ranges
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let heading_by_id = snapshot
            .headings
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let list_by_id = snapshot
            .lists
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let footnote_definition_by_id = snapshot
            .footnote_definitions
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let code_block_by_id = snapshot
            .code_blocks
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let table_by_id = snapshot
            .tables
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let image_by_id = snapshot
            .images
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let link_by_id = snapshot
            .links
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let task_by_id = snapshot
            .tasks
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();
        let wiki_by_id = snapshot
            .wikis
            .into_iter()
            .map(|value| (value.node_id, value))
            .collect::<HashMap<_, _>>();

        let mut nodes_by_id = HashMap::new();

        for base in snapshot.bases {
            let node_id = base.id;
            let record = NodeRecord {
                base,
                text: text_by_id.get(&node_id).cloned(),
                range: range_by_id.get(&node_id).cloned(),
                heading: heading_by_id.get(&node_id).cloned(),
                footnote_definition: footnote_definition_by_id.get(&node_id).cloned(),
                list: list_by_id.get(&node_id).cloned(),
                code_block: code_block_by_id.get(&node_id).cloned(),
                table: table_by_id.get(&node_id).cloned(),
                image: image_by_id.get(&node_id).cloned(),
                link: link_by_id.get(&node_id).cloned(),
                task: task_by_id.get(&node_id).cloned(),
                wiki: wiki_by_id.get(&node_id).cloned(),
            };
            nodes_by_id.insert(node_id, record);
        }

        Ok(nodes_by_id)
    }
}
