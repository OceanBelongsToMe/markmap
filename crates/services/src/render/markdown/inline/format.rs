use crate::render::markdown::inline::context::InlineRecordView;

pub trait InlineFormat {
    fn text(&self, content: &str) -> String {
        content.to_string()
    }
    fn emphasis(&self, content: &str) -> String;
    fn strong(&self, content: &str) -> String;
    fn strikethrough(&self, content: &str) -> String;
    fn superscript(&self, content: &str) -> String;
    fn subscript(&self, content: &str) -> String;
    fn code_inline(&self, content: &str) -> String;
    fn math_inline(&self, content: &str) -> String;
    fn html_inline(&self, content: &str) -> String {
        content.to_string()
    }
    fn footnote_reference(&self, content: &str) -> String;
    fn link(&self, record: &dyn InlineRecordView, content: &str) -> String;
    fn image(&self, record: &dyn InlineRecordView, content: &str) -> String;
    fn wiki(&self, record: &dyn InlineRecordView, content: &str) -> String;
}

pub struct MarkdownInlineFormat;

impl MarkdownInlineFormat {
    pub fn new() -> Self {
        Self
    }
}

impl InlineFormat for MarkdownInlineFormat {
    fn emphasis(&self, content: &str) -> String {
        format!("*{content}*")
    }

    fn strong(&self, content: &str) -> String {
        format!("**{content}**")
    }

    fn strikethrough(&self, content: &str) -> String {
        format!("~~{content}~~")
    }

    fn superscript(&self, content: &str) -> String {
        format!("^{content}^")
    }

    fn subscript(&self, content: &str) -> String {
        format!("~{content}~")
    }

    fn code_inline(&self, content: &str) -> String {
        format!("`{content}`")
    }

    fn math_inline(&self, content: &str) -> String {
        format!("${content}$")
    }

    fn footnote_reference(&self, content: &str) -> String {
        format!("[^{content}]")
    }

    fn link(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(link) = record.link() {
            let label = if content.trim().is_empty() {
                link.href.as_str()
            } else {
                content
            };
            let mut line = format!("[{label}]({}", link.href);
            if let Some(title) = link.title.as_ref() {
                line.push_str(&format!(" \"{}\"", title));
            }
            line.push(')');
            line
        } else {
            content.to_string()
        }
    }

    fn image(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(image) = record.image() {
            let alt = image.alt.as_deref().unwrap_or("");
            let mut line = format!("![{alt}]({}", image.src);
            if let Some(title) = image.title.as_ref() {
                line.push_str(&format!(" \"{}\"", title));
            }
            line.push(')');
            line
        } else {
            content.to_string()
        }
    }

    fn wiki(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(wiki) = record.wiki() {
            let target = wiki.target_node_id.as_uuid();
            let display = wiki.display_text.trim();
            if display.is_empty() {
                format!("[[{target}]]")
            } else {
                format!("[[{display}|{target}]]")
            }
        } else {
            content.to_string()
        }
    }
}

pub struct HtmlInlineFormat;

impl HtmlInlineFormat {
    pub fn new() -> Self {
        Self
    }
}

impl InlineFormat for HtmlInlineFormat {
    fn emphasis(&self, content: &str) -> String {
        format!("<em>{content}</em>")
    }

    fn strong(&self, content: &str) -> String {
        format!("<strong>{content}</strong>")
    }

    fn strikethrough(&self, content: &str) -> String {
        format!("<del>{content}</del>")
    }

    fn superscript(&self, content: &str) -> String {
        format!("<sup>{content}</sup>")
    }

    fn subscript(&self, content: &str) -> String {
        format!("<sub>{content}</sub>")
    }

    fn code_inline(&self, content: &str) -> String {
        format!("<code>{content}</code>")
    }

    fn math_inline(&self, content: &str) -> String {
        format!("<span class=\"math-inline\">{content}</span>")
    }

    fn footnote_reference(&self, content: &str) -> String {
        format!("<sup class=\"footnote-ref\">{content}</sup>")
    }

    fn link(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(link) = record.link() {
            let label = if content.trim().is_empty() {
                link.href.as_str()
            } else {
                content
            };
            if let Some(title) = link.title.as_ref() {
                format!("<a href=\"{}\" title=\"{}\">{label}</a>", link.href, title)
            } else {
                format!("<a href=\"{}\">{label}</a>", link.href)
            }
        } else {
            content.to_string()
        }
    }

    fn image(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(image) = record.image() {
            let alt = image.alt.as_deref().unwrap_or("");
            if let Some(title) = image.title.as_ref() {
                format!(
                    "<img src=\"{}\" alt=\"{}\" title=\"{}\" />",
                    image.src, alt, title
                )
            } else {
                format!("<img src=\"{}\" alt=\"{}\" />", image.src, alt)
            }
        } else {
            content.to_string()
        }
    }

    fn wiki(&self, record: &dyn InlineRecordView, content: &str) -> String {
        if let Some(wiki) = record.wiki() {
            let target = wiki.target_node_id.as_uuid();
            let display = wiki.display_text.trim();
            let label = if display.is_empty() {
                target.to_string()
            } else {
                display.to_string()
            };
            format!("<span class=\"wiki\" data-target=\"{target}\">{label}</span>")
        } else {
            content.to_string()
        }
    }
}
