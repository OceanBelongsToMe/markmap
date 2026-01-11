use common::types::AppResult;

pub trait HtmlPostProcessor: Send + Sync {
    fn process(&self, html: &str) -> AppResult<String>;
}

pub struct NoopPostProcessor;

impl HtmlPostProcessor for NoopPostProcessor {
    fn process(&self, html: &str) -> AppResult<String> {
        Ok(html.to_string())
    }
}
