use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::domain::query::{Fragment, Hit, QueryEngine, QueryInput};

pub struct SearchQuery<'a> {
    engine: &'a dyn QueryEngine,
}

impl<'a> SearchQuery<'a> {
    pub fn new(engine: &'a dyn QueryEngine) -> Self {
        Self { engine }
    }

    pub fn execute(&self, input: QueryInput) -> AppResult<Vec<Hit>> {
        self.engine.search(input)
    }

    pub fn highlights(&self, doc_id: DocumentId, query: String) -> AppResult<Vec<Fragment>> {
        self.engine.highlights(doc_id, query)
    }
}
