use std::collections::VecDeque;

use knowlattice_core::model::DocumentId;

use crate::domain::indexer::IndexTask;

#[derive(Debug, Default)]
pub struct IndexQueue {
    tasks: VecDeque<IndexTask>,
}

impl IndexQueue {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn push(&mut self, task: IndexTask) {
        self.tasks.push_back(task);
    }

    pub fn pop(&mut self) -> Option<IndexTask> {
        self.tasks.pop_front()
    }

    pub fn merge_by_doc(&mut self, doc_id: DocumentId, task: IndexTask) {
        if let Some(pos) = self.tasks.iter().position(|item| item.doc_id == doc_id) {
            self.tasks.remove(pos);
        }
        self.tasks.push_back(task);
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
