use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::error::ApiError;

use super::handler::CommandHandler;

pub trait CommandCodec: Send + Sync {
    fn parse(&self, payload: Value) -> Result<Box<dyn std::any::Any + Send>, ApiError>;
    fn serialize(&self, response: Box<dyn std::any::Any + Send>) -> Result<Value, ApiError>;
}

pub(crate) struct SerdeCodec<H: CommandHandler> {
    _phantom: std::marker::PhantomData<H>,
}

impl<H: CommandHandler> SerdeCodec<H> {
    pub(crate) fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<H> CommandCodec for SerdeCodec<H>
where
    H: CommandHandler + 'static,
{
    fn parse(&self, payload: Value) -> Result<Box<dyn std::any::Any + Send>, ApiError> {
        let request: H::Request = serde_json::from_value(payload).map_err(|err| {
            ApiError::with_details("INVALID_PAYLOAD", "invalid payload", err.to_string())
        })?;
        Ok(Box::new(request))
    }

    fn serialize(&self, response: Box<dyn std::any::Any + Send>) -> Result<Value, ApiError> {
        let response = response
            .downcast::<H::Response>()
            .map_err(|_| ApiError::new("INVALID_RESPONSE", "invalid response type"))?;
        serde_json::to_value(*response).map_err(|err| {
            ApiError::with_details("INVALID_RESPONSE", "invalid response", err.to_string())
        })
    }
}

pub struct CodecRegistry {
    codecs: HashMap<String, Arc<dyn CommandCodec>>,
}

impl CodecRegistry {
    pub fn new() -> Self {
        Self {
            codecs: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, name: &str)
    where
        H: CommandHandler + 'static,
    {
        self.codecs
            .insert(name.to_string(), Arc::new(SerdeCodec::<H>::new()));
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Arc<dyn CommandCodec>> {
        self.codecs.get(name)
    }
}
