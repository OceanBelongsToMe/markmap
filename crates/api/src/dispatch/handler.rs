use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::ApiError;

use super::context::ApiContext;

#[async_trait]
pub trait CommandHandler: Send + Sync {
    type Request: DeserializeOwned + Send + 'static;
    type Response: Serialize + Send + 'static;

    fn name(&self) -> &'static str;
    async fn handle(
        &self,
        ctx: &ApiContext,
        payload: Self::Request,
    ) -> Result<Self::Response, ApiError>;
}

#[async_trait]
pub trait DynCommandHandler: Send + Sync {
    fn name(&self) -> &'static str;
    async fn handle(
        &self,
        ctx: &ApiContext,
        request: Box<dyn std::any::Any + Send>,
    ) -> Result<Box<dyn std::any::Any + Send>, ApiError>;
}

pub(crate) struct HandlerAdapter<H: CommandHandler> {
    pub(crate) handler: H,
}

#[async_trait]
impl<H> DynCommandHandler for HandlerAdapter<H>
where
    H: CommandHandler + Send + Sync + 'static,
{
    fn name(&self) -> &'static str {
        self.handler.name()
    }

    async fn handle(
        &self,
        ctx: &ApiContext,
        request: Box<dyn std::any::Any + Send>,
    ) -> Result<Box<dyn std::any::Any + Send>, ApiError> {
        let request = request
            .downcast::<H::Request>()
            .map_err(|_| ApiError::new("INVALID_PAYLOAD", "invalid payload type"))?;
        let response = self.handler.handle(ctx, *request).await?;
        Ok(Box::new(response))
    }
}
