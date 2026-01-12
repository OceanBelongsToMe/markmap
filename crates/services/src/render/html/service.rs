use std::sync::Arc;

use common::types::AppResult;
use knowlattice_core::model::DocumentId;

use crate::builder::ServiceRegistry;
use crate::render::RenderOutput;
use crate::render::html::postprocess::{HtmlPostProcessor, NoopPostProcessor};
use crate::render::html::renderer::{ComrakRenderer, MarkdownToHtml};
use crate::render::html::sanitizer::{AmmoniaSanitizer, HtmlSanitizer};
use crate::render::html::source::{MarkdownSourceProvider, RenderMarkdownSource};
use crate::render::markdown::RenderMarkdown;

pub struct RenderHtml {
    source: Arc<dyn MarkdownSourceProvider>,
    renderer: Arc<dyn MarkdownToHtml>,
    postprocessors: Vec<Arc<dyn HtmlPostProcessor>>,
    sanitizer: Option<Arc<dyn HtmlSanitizer>>,
}

impl RenderHtml {
    pub fn register(registry: &mut ServiceRegistry) {
        let markdown: Arc<RenderMarkdown> = registry.get().expect("render markdown");
        let source: Arc<dyn MarkdownSourceProvider> =
            Arc::new(RenderMarkdownSource::new(markdown));
        let renderer: Arc<dyn MarkdownToHtml> = Arc::new(ComrakRenderer::new());
        let postprocessors: Vec<Arc<dyn HtmlPostProcessor>> =
            vec![Arc::new(NoopPostProcessor)];
        let sanitizer: Option<Arc<dyn HtmlSanitizer>> = if sanitize_html_env() {
            Some(Arc::new(AmmoniaSanitizer::new()))
        } else {
            None
        };

        registry.register(Arc::new(RenderHtml {
            source,
            renderer,
            postprocessors,
            sanitizer,
        }));
    }

    pub async fn execute(&self, doc_id: DocumentId) -> AppResult<RenderOutput> {
        let markdown = self.source.load_markdown(doc_id).await?;
        let mut html = self.renderer.render(&markdown)?;

        for processor in &self.postprocessors {
            html = processor.process(&html)?;
        }

        if let Some(sanitizer) = &self.sanitizer {
            html = sanitizer.sanitize(&html)?;
        }

        Ok(RenderOutput::Text(html))
    }
}

fn sanitize_html_env() -> bool {
    std::env::var("KNOWLATTICE_RENDER_HTML_SANITIZE")
        .ok()
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::RenderHtml;
    use crate::render::RenderOutput;
    use crate::render::html::postprocess::HtmlPostProcessor;
    use crate::render::html::renderer::MarkdownToHtml;
    use crate::render::html::sanitizer::HtmlSanitizer;
    use crate::render::html::source::MarkdownSourceProvider;
    use common::types::AppResult;
    use knowlattice_core::model::DocumentId;
    use std::sync::{Arc, Mutex};

    struct StubSource {
        trace: Arc<Mutex<Vec<&'static str>>>,
    }

    #[async_trait::async_trait]
    impl MarkdownSourceProvider for StubSource {
        async fn load_markdown(&self, _doc_id: DocumentId) -> AppResult<String> {
            self.trace.lock().expect("trace").push("source");
            Ok("**md**".to_string())
        }
    }

    struct StubRenderer {
        trace: Arc<Mutex<Vec<&'static str>>>,
    }

    impl MarkdownToHtml for StubRenderer {
        fn render(&self, markdown: &str) -> AppResult<String> {
            self.trace.lock().expect("trace").push("renderer");
            Ok(format!("<p>{markdown}</p>"))
        }
    }

    struct StubPostProcessor {
        trace: Arc<Mutex<Vec<&'static str>>>,
    }

    impl HtmlPostProcessor for StubPostProcessor {
        fn process(&self, html: &str) -> AppResult<String> {
            self.trace.lock().expect("trace").push("post");
            Ok(format!("{html}<!--post-->"))
        }
    }

    struct StubSanitizer {
        trace: Arc<Mutex<Vec<&'static str>>>,
    }

    impl HtmlSanitizer for StubSanitizer {
        fn sanitize(&self, html: &str) -> AppResult<String> {
            self.trace.lock().expect("trace").push("sanitize");
            Ok(format!("<!--sanitized-->{html}"))
        }
    }

    #[tokio::test]
    async fn render_html_pipeline_orders_steps() {
        let trace = Arc::new(Mutex::new(Vec::new()));
        let service = RenderHtml {
            source: Arc::new(StubSource {
                trace: Arc::clone(&trace),
            }),
            renderer: Arc::new(StubRenderer {
                trace: Arc::clone(&trace),
            }),
            postprocessors: vec![Arc::new(StubPostProcessor {
                trace: Arc::clone(&trace),
            })],
            sanitizer: Some(Arc::new(StubSanitizer {
                trace: Arc::clone(&trace),
            })),
        };

        let output = service
            .execute(DocumentId::new())
            .await
            .expect("execute");

        assert_eq!(
            trace.lock().expect("trace").as_slice(),
            ["source", "renderer", "post", "sanitize"]
        );

        match output {
            RenderOutput::Text(html) => {
                assert!(html.contains("<!--sanitized-->"));
                assert!(html.contains("<!--post-->"));
            }
            _ => panic!("expected html text output"),
        }
    }
}
