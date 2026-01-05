use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::dispatch::{ApiContext, CodecRegistry, CommandHandler, CommandRegistry};
use crate::error::ApiError;

pub struct PingHandler<Req, Res> {
    name: &'static str,
    _phantom: PhantomData<fn(Req) -> Res>,
}

impl<Req, Res> PingHandler<Req, Res> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            _phantom: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<Req, Res> CommandHandler for PingHandler<Req, Res>
where
    Req: DeserializeOwned + Send + 'static,
    Res: Serialize + Default + Send + 'static,
{
    type Request = Req;
    type Response = Res;

    fn name(&self) -> &'static str {
        self.name
    }

    async fn handle(&self, _ctx: &ApiContext, _payload: Req) -> Result<Res, ApiError> {
        Ok(Res::default())
    }
}

pub fn register_ping<Req, Res>(registry: &mut CommandRegistry, name: &'static str)
where
    Req: DeserializeOwned + Send + 'static,
    Res: Serialize + Default + Send + 'static,
{
    registry.register(PingHandler::<Req, Res>::new(name));
}

pub fn register_ping_codec<Req, Res>(codecs: &mut CodecRegistry, name: &'static str)
where
    Req: DeserializeOwned + Send + 'static,
    Res: Serialize + Default + Send + 'static,
{
    codecs.register::<PingHandler<Req, Res>>(name);
}
