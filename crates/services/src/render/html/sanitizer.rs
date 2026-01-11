use common::types::AppResult;

pub trait HtmlSanitizer: Send + Sync {
    fn sanitize(&self, html: &str) -> AppResult<String>;
}

pub struct NoopSanitizer;

impl HtmlSanitizer for NoopSanitizer {
    fn sanitize(&self, html: &str) -> AppResult<String> {
        Ok(html.to_string())
    }
}
