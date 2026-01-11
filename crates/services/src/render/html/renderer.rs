use common::error::{AppError, ErrorCode};
use common::types::AppResult;
use comrak::{markdown_to_html, Options};

pub trait MarkdownToHtml: Send + Sync {
    fn render(&self, markdown: &str) -> AppResult<String>;
}

pub struct UnconfiguredMarkdownToHtml;

impl MarkdownToHtml for UnconfiguredMarkdownToHtml {
    fn render(&self, _markdown: &str) -> AppResult<String> {
        Err(AppError::new(
            ErrorCode::Internal,
            "render html not configured",
        ))
    }
}

pub struct ComrakRenderer {
    options: Options,
}

impl ComrakRenderer {
    pub fn new() -> Self {
        let mut options = Options::default();
        options.render.unsafe_ = true;
        options.extension.table = true;
        options.extension.strikethrough = true;
        options.extension.tasklist = true;
        options.extension.autolink = true;
        options.extension.superscript = true;
        options.extension.footnotes = true;
        Self { options }
    }
}

impl MarkdownToHtml for ComrakRenderer {
    fn render(&self, markdown: &str) -> AppResult<String> {
        Ok(markdown_to_html(markdown, &self.options))
    }
}
