pub const FTS_TABLE: &str = "search_fts";

pub const FTS_SCHEMA_SQL: &str = "\
CREATE VIRTUAL TABLE IF NOT EXISTS search_fts USING fts5(\
    node_id UNINDEXED,\
    doc_id UNINDEXED,\
    node_type,\
    title_path,\
    text\
);\
";
