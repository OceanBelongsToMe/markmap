use std::collections::VecDeque;

use super::parser::ParseTask;
use knowlattice_core::model::DocumentId;

#[derive(Debug, Default)]
pub struct ParseQueue {
    tasks: VecDeque<ParseTask>,
}

impl ParseQueue {
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    pub fn push(&mut self, task: ParseTask) {
        self.tasks.push_back(task);
    }

    pub fn pop(&mut self) -> Option<ParseTask> {
        self.tasks.pop_front()
    }

    pub fn merge_by_doc(&mut self, doc_id: DocumentId, task: ParseTask) {
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
