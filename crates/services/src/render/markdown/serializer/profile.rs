use crate::render::markdown::inline::format::MarkdownInlineFormat;

pub struct MarkdownStyleProfile {
    inline_format: MarkdownInlineFormat,
}

impl MarkdownStyleProfile {
    pub fn new() -> Self {
        Self {
            inline_format: MarkdownInlineFormat::new(),
        }
    }

    pub fn inline_format(&self) -> &MarkdownInlineFormat {
        &self.inline_format
    }
}
