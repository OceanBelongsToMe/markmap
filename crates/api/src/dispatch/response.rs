use serde_json::{json, Value};

use crate::dto::DtoResponse;
use crate::error::ApiError;

use super::context::ApiContext;

pub struct ResponseMapper;

impl ResponseMapper {
    pub fn ok(ctx: &ApiContext, data: Value) -> DtoResponse {
        DtoResponse {
            ok: true,
            data,
            error: None,
            request_id: ctx
                .request_id
                .clone()
                .unwrap_or_else(|| ctx.trace_id.clone()),
        }
    }

    pub fn error(ctx: &ApiContext, err: ApiError) -> DtoResponse {
        DtoResponse {
            ok: false,
            data: json!({}),
            error: Some(err.with_trace_id(ctx.trace_id.clone())),
            request_id: ctx
                .request_id
                .clone()
                .unwrap_or_else(|| ctx.trace_id.clone()),
        }
    }
}
