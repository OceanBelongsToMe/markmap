use std::sync::Arc;

use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use knowlattice_core::model::DocumentId;
use tokio::sync::{mpsc, Mutex};

#[derive(Debug)]
pub struct IndexQueue {
    sender: mpsc::Sender<DocumentId>,
    receiver: Arc<Mutex<mpsc::Receiver<DocumentId>>>,
}

impl IndexQueue {
    pub fn new(buffer: usize) -> Self {
        let (sender, receiver) = mpsc::channel(buffer);
        Self {
            sender,
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub async fn enqueue(&self, doc_id: DocumentId) -> AppResult<()> {
        self.sender
            .send(doc_id)
            .await
            .map_err(|_| AppError::new(ErrorCode::Internal, "index queue send failed"))
    }

    pub async fn next(&self) -> Option<DocumentId> {
        let mut receiver = self.receiver.lock().await;
        receiver.recv().await
    }

    pub fn sender(&self) -> mpsc::Sender<DocumentId> {
        self.sender.clone()
    }
}
