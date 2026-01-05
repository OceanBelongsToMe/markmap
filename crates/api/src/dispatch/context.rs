use std::sync::Arc;

use common::log::TraceId;
use knowlattice_services::builder::Services;

use crate::dto::DtoRequest;

#[derive(Clone)]
pub struct ApiContext {
    pub request_id: Option<String>,
    pub trace_id: String,
    pub services: Arc<Services>,
}

pub(crate) struct ApiContextBuilder;

impl ApiContextBuilder {
    pub(crate) fn build(services: Arc<Services>, req: &DtoRequest) -> ApiContext {
        let request_id = req.request_id.clone();
        let trace_id_value = request_id
            .as_deref()
            .and_then(|value| TraceId::parse(value).ok())
            .unwrap_or_else(TraceId::new);
        let trace_id = trace_id_value.to_string();
        ApiContext {
            request_id,
            trace_id,
            services,
        }
    }
}
