use std::sync::Arc;

use async_trait::async_trait;

use crate::error::ApiError;

use super::context::ApiContext;

pub type ParsedRequest = Box<dyn std::any::Any + Send>;
pub type RawResponse = Box<dyn std::any::Any + Send>;

#[async_trait]
pub trait PreMiddleware: Send + Sync {
    async fn handle(
        &self,
        ctx: &ApiContext,
        request: ParsedRequest,
        next: PreNext<'_>,
    ) -> Result<ParsedRequest, ApiError>;
}

#[async_trait]
pub trait PostMiddleware: Send + Sync {
    async fn handle(
        &self,
        ctx: &ApiContext,
        response: RawResponse,
        next: PostNext<'_>,
    ) -> Result<RawResponse, ApiError>;
}

pub struct PrePipeline {
    middlewares: Vec<Arc<dyn PreMiddleware>>,
}

pub struct PostPipeline {
    middlewares: Vec<Arc<dyn PostMiddleware>>,
}

pub struct LoggingPreMiddleware;

#[async_trait]
impl PreMiddleware for LoggingPreMiddleware {
    async fn handle(
        &self,
        ctx: &ApiContext,
        request: ParsedRequest,
        next: PreNext<'_>,
    ) -> Result<ParsedRequest, ApiError> {
        use common::log::{span, LogContext, SpanName, TraceId};
        let trace_id_value = TraceId::parse(&ctx.trace_id).unwrap_or_else(|_| TraceId::new());
        let log_ctx = LogContext::new(trace_id_value);
        let span = span(&log_ctx, SpanName::Operation);
        let _guard = span.enter();
        next.run(ctx, request).await
    }
}

impl PrePipeline {
    pub(crate) fn new(middlewares: Vec<Arc<dyn PreMiddleware>>) -> Self {
        Self { middlewares }
    }

    pub(crate) async fn run(
        &self,
        ctx: &ApiContext,
        request: ParsedRequest,
    ) -> Result<ParsedRequest, ApiError> {
        let next = PreNext {
            middlewares: &self.middlewares,
        };
        next.run(ctx, request).await
    }
}

impl PostPipeline {
    pub(crate) fn new(middlewares: Vec<Arc<dyn PostMiddleware>>) -> Self {
        Self { middlewares }
    }

    pub(crate) async fn run(
        &self,
        ctx: &ApiContext,
        response: RawResponse,
    ) -> Result<RawResponse, ApiError> {
        let next = PostNext {
            middlewares: &self.middlewares,
        };
        next.run(ctx, response).await
    }
}

pub struct PreNext<'a> {
    middlewares: &'a [Arc<dyn PreMiddleware>],
}

impl<'a> PreNext<'a> {
    async fn run(
        &self,
        ctx: &ApiContext,
        request: ParsedRequest,
    ) -> Result<ParsedRequest, ApiError> {
        if let Some((first, rest)) = self.middlewares.split_first() {
            let next = PreNext {
                middlewares: rest,
            };
            return first.handle(ctx, request, next).await;
        }
        Ok(request)
    }
}

pub struct PostNext<'a> {
    middlewares: &'a [Arc<dyn PostMiddleware>],
}

impl<'a> PostNext<'a> {
    async fn run(
        &self,
        ctx: &ApiContext,
        response: RawResponse,
    ) -> Result<RawResponse, ApiError> {
        if let Some((first, rest)) = self.middlewares.split_first() {
            let next = PostNext {
                middlewares: rest,
            };
            return first.handle(ctx, response, next).await;
        }
        Ok(response)
    }
}
