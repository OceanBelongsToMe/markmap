### 3.4 search（索引与查询）

- 责任边界：解析、索引构建与检索能力。
- 不负责：业务用例编排与存储装配。
- 目录映射：`crates/search/`。
- 交付物：端口定义、用例与适配器实现说明。
- 验收指标：检索准确性、可替换性。

## 架构概览

- 三层结构：`domain` / `application` / `adapters`
- 目标：接口稳定、流程清晰、实现可替换

## 子模块与组成（高层）

- `search::domain`：端口与数据结构（Parser/Indexer/QueryEngine/SchemaExecutor trait）
- `search::application`：用例与队列（IndexDocument / SearchQuery / ParseQueue / IndexQueue）
- `search::adapters`：实现层（markdown/sqlite/null）

## 路径索引（实现入口）

- 端口与结构：`crates/search/src/domain/`
- 用例与队列：`crates/search/src/application/`
- 适配器实现：`crates/search/src/adapters/`

## 少量示例（方便快速理解）

- 解析入口：MarkdownParser（adapters/markdown）
- 查询入口：SearchQuery（application/usecases）
- SQLite 实现：adapters/sqlite
