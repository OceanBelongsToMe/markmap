use std::sync::Arc;

use ammonia::Builder;
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

pub struct AmmoniaSanitizer {
    builder: Arc<Builder<'static>>,
}

impl AmmoniaSanitizer {
    pub fn new() -> Self {
        Self {
            builder: Arc::new(Builder::default()),
        }
    }
}

impl HtmlSanitizer for AmmoniaSanitizer {
    fn sanitize(&self, html: &str) -> AppResult<String> {
        Ok(self.builder.clean(html).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::AmmoniaSanitizer;
    use super::HtmlSanitizer;

    #[test]
    fn sanitizer_strips_script_tags() {
        let sanitizer = AmmoniaSanitizer::new();
        let html = "<p>ok</p><script>alert(1)</script>";
        let sanitized = sanitizer.sanitize(html).expect("sanitize");
        assert!(sanitized.contains("<p>ok</p>"));
        assert!(!sanitized.contains("<script>"));
    }
}
