use std::sync::Arc;

use knowlattice_services::builder::Services;

use crate::dto::{DtoRequest, DtoResponse};
use crate::error::ApiError;

use super::codec::CodecRegistry;
use super::context::ApiContextBuilder;
use super::pipeline::{LoggingPreMiddleware, PostPipeline, PrePipeline};
use super::response::ResponseMapper;
use super::registry::CommandRegistry;

pub struct CommandRouter {
    registry: Arc<CommandRegistry>,
    codecs: Arc<CodecRegistry>,
    pre_middlewares: Vec<Arc<dyn super::pipeline::PreMiddleware>>,
    post_middlewares: Vec<Arc<dyn super::pipeline::PostMiddleware>>,
}

impl CommandRouter {
    pub fn with_codecs(registry: Arc<CommandRegistry>, codecs: Arc<CodecRegistry>) -> Self {
        Self {
            registry,
            codecs,
            pre_middlewares: vec![Arc::new(LoggingPreMiddleware)],
            post_middlewares: Vec::new(),
        }
    }

    pub async fn dispatch(&self, services: Arc<Services>, req: DtoRequest) -> DtoResponse {
        let ctx = ApiContextBuilder::build(services, &req);
        let Some(handler) = self.registry.get(&req.command) else {
            return ResponseMapper::error(&ctx, ApiError::new("NOT_FOUND", "command not found"));
        };
        let Some(codec) = self.codecs.get(&req.command) else {
            return ResponseMapper::error(&ctx, ApiError::new("NOT_FOUND", "codec not found"));
        };

        let pre_pipeline = PrePipeline::new(self.pre_middlewares.clone());
        let post_pipeline = PostPipeline::new(self.post_middlewares.clone());
        let parsed = match codec.parse(req.payload.clone()) {
            Ok(value) => value,
            Err(err) => return ResponseMapper::error(&ctx, err),
        };
        let result = match pre_pipeline.run(&ctx, parsed).await {
            Ok(parsed) => {
                let response = handler.handle(&ctx, parsed).await;
                match response {
                    Ok(raw) => post_pipeline.run(&ctx, raw).await,
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        };
        let raw = match result {
            Ok(raw) => raw,
            Err(err) => return ResponseMapper::error(&ctx, err),
        };
        match codec.serialize(raw) {
            Ok(data) => ResponseMapper::ok(&ctx, data),
            Err(err) => ResponseMapper::error(&ctx, err),
        }
    }
}
