use super::rules::ensure_blank_line;

pub trait SpacingPolicy: Send + Sync {
    fn ensure_blank_line(&self, out: &mut Vec<String>);
    fn trim_trailing_blank_lines(&self, out: &mut Vec<String>);
    fn before_horizontal_rule(&self, out: &mut Vec<String>);
}

pub struct DefaultSpacingPolicy;

impl DefaultSpacingPolicy {
    pub fn new() -> Self {
        Self
    }
}

impl SpacingPolicy for DefaultSpacingPolicy {
    fn ensure_blank_line(&self, out: &mut Vec<String>) {
        ensure_blank_line(out);
    }

    fn trim_trailing_blank_lines(&self, out: &mut Vec<String>) {
        while matches!(out.last(), Some(last) if last.is_empty()) {
            out.pop();
        }
    }

    fn before_horizontal_rule(&self, out: &mut Vec<String>) {
        if matches!(out.last(), Some(last) if last.is_empty()) {
            out.pop();
        }
    }
}
