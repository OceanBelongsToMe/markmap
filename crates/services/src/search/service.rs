use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::{DocumentId, FolderId, NodeId, WorkspaceId};

use std::sync::Arc;

use crate::builder::ServiceRegistry;

#[derive(Debug, Clone)]
pub enum SearchScope {
    Document(DocumentId),
    Folder(FolderId),
    Workspace(WorkspaceId),
}

#[derive(Debug, Clone)]
pub struct SearchHit {
    pub node_id: NodeId,
    pub path: String,
    pub snippet: String,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub struct HighlightFragment {
    pub node_id: NodeId,
    pub ranges: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
pub struct NodeDetails {
    pub node_id: NodeId,
    pub text: String,
    pub node_type: String,
}

pub struct Search;

impl Search {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(Search));
    }

    pub async fn execute(
        &self,
        _query: String,
        _scope: SearchScope,
        _limit: usize,
        _offset: usize,
    ) -> AppResult<Vec<SearchHit>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search service not configured",
        ))
    }
}

pub struct GetNodeTree;

impl GetNodeTree {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(GetNodeTree));
    }

    pub async fn execute(&self, _doc_id: DocumentId) -> AppResult<Vec<NodeId>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search service not configured",
        ))
    }
}

pub struct QueryHighlights;

impl QueryHighlights {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(QueryHighlights));
    }

    pub async fn execute(
        &self,
        _doc_id: DocumentId,
        _query: String,
    ) -> AppResult<Vec<HighlightFragment>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search service not configured",
        ))
    }
}

pub struct GetNodeDetails;

impl GetNodeDetails {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(GetNodeDetails));
    }

    pub async fn execute(&self, _node_id: NodeId) -> AppResult<NodeDetails> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search service not configured",
        ))
    }
}

pub struct GetSearchSuggestions;

impl GetSearchSuggestions {
    pub fn register(registry: &mut ServiceRegistry) {
        registry.register(Arc::new(GetSearchSuggestions));
    }

    pub async fn execute(&self, _query: String) -> AppResult<Vec<String>> {
        Err(AppError::new(
            ErrorCode::Internal,
            "search service not configured",
        ))
    }
}
