use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceId(Uuid);

impl TraceId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn parse(value: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(value)?))
    }

    pub fn as_uuid(self) -> Uuid {
        self.0
    }
}

impl std::fmt::Display for TraceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpanName {
    Operation,
    ConfigLoad,
    ConfigParse,
    StorageRead,
    StorageWrite,
    ServiceCall,
}

#[derive(Debug, Clone)]
pub struct LogContext {
    pub trace_id: TraceId,
}

impl LogContext {
    pub fn new(trace_id: TraceId) -> Self {
        Self { trace_id }
    }
}

pub fn span(ctx: &LogContext, name: SpanName) -> tracing::Span {
    macro_rules! mk_span {
        ($span_name:literal) => {
            tracing::info_span!($span_name, trace_id = %ctx.trace_id)
        };
    }

    match name {
        SpanName::Operation => mk_span!("operation"),
        SpanName::ConfigLoad => mk_span!("config.load"),
        SpanName::ConfigParse => mk_span!("config.parse"),
        SpanName::StorageRead => mk_span!("storage.read"),
        SpanName::StorageWrite => mk_span!("storage.write"),
        SpanName::ServiceCall => mk_span!("service.call"),
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {
        tracing::info!($($arg)+)
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {
        tracing::warn!($($arg)+)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {
        tracing::error!($($arg)+)
    };
}
