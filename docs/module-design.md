# 项目模块设计文档（Rust 后端）

## 1. 目标与范围

- 目标：为基于 markmap 的 Markdown 编辑软件提供可维护、可扩展的后端结构
- 范围：Rust 后端（Tauri 侧），采用 workspace + 多 crate 组织

## 2. 总体架构

- 采用 Rust workspace，Tauri 入口仅做 wiring
- 业务逻辑与存储、索引、导出解耦
- 以 Workspace 为核心边界（聚合根），支持多个根目录（Folder）

## 3. 目录与模块职责

### 3.0 目录结构管理

```
.
├─ docs/
├─ public/
├─ src/
│  └─ assets/
├─ src-tauri/
│  ├─ capabilities/
│  ├─ gen/
│  ├─ icons/
│  ├─ src/               # Tauri 入口与后端代码
│  │  ├─ main.rs
│  │  ├─ lib.rs
│  │  ├─ commands/
│  │  │  ├─ mod.rs
│  │  ├─ dto/
│  │  │  ├─ mod.rs
│  │  │  └─ re-export crates/api::dto
│  │  └─ error.rs
│  └─ target/
├─ crates/               # 规划目录（Rust workspace）
│  ├─ core/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ model/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ workspace.rs
│  │  │  │  ├─ folder.rs
│  │  │  │  ├─ document.rs
│  │  │  │  ├─ node_base.rs
│  │  │  │  └─ node_type.rs
│  │  │  ├─ policy/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ validate_workspace.rs
│  │  │  │  ├─ validate_document.rs
│  │  │  │  ├─ validate_node_tree.rs
│  │  │  │  └─ validate_path.rs
│  │  │  ├─ event/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ events.rs
│  │  │  └─ error/
│  │  │     ├─ mod.rs
│  │  │     └─ domain_error.rs
│  ├─ services/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ workspace/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  ├─ config/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  ├─ document/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  ├─ index/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  ├─ render/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  ├─ search/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ service.rs
│  │  │  └─ export/
│  │  │     ├─ mod.rs
│  │  │     └─ service.rs
│  ├─ storage/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ repo/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ workspace_repo.rs
│  │  │  │  ├─ folder_repo.rs
│  │  │  │  ├─ document_repo.rs
│  │  │  │  └─ node/
│  │  │  │     ├─ mod.rs
│  │  │  │     ├─ node_base_repo.rs
│  │  │  │     └─ node_*_repo.rs
│  │  │  ├─ fs/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ fs_storage.rs
│  │  │  ├─ sqlite/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ pool.rs
│  │  │  │  ├─ mapper.rs
│  │  │  │  └─ repo_impl.rs
│  │  │  └─ migrate/
│  │  │     ├─ mod.rs
│  │  │     └─ migrations.rs
│  ├─ search/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ parser/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ parser.rs
│  │  │  │  ├─ markdown_parser.rs
│  │  │  │  └─ parse_queue.rs
│  │  │  ├─ indexer/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ indexer.rs
│  │  │  │  ├─ sqlite_indexer.rs
│  │  │  │  └─ index_queue.rs
│  │  │  ├─ query/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ query.rs
│  │  │  │  └─ sqlite_query.rs
│  │  │  └─ schema/
│  │  │     ├─ mod.rs
│  │  │     ├─ fts_schema.rs
│  │  │     └─ migrate.rs
│  ├─ export/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ pipeline/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ pipeline.rs
│  │  │  ├─ format/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ markdown.rs
│  │  │  │  ├─ html.rs
│  │  │  │  ├─ svg.rs
│  │  │  │  ├─ png.rs
│  │  │  │  └─ pdf.rs
│  │  │  └─ theme/
│  │  │     ├─ mod.rs
│  │  │     └─ theme.rs
│  ├─ api/               # 共享 API 层（跨 Tauri/CLI 等复用）
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ command/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ workspace.rs
│  │  │  │  ├─ folder.rs
│  │  │  │  ├─ document.rs
│  │  │  │  ├─ search.rs
│  │  │  │  ├─ index.rs
│  │  │  │  ├─ render.rs
│  │  │  │  └─ export.rs
│  │  │  ├─ dto/
│  │  │  │  ├─ mod.rs
│  │  │  │  ├─ workspace.rs
│  │  │  │  ├─ folder.rs
│  │  │  │  ├─ document.rs
│  │  │  │  ├─ search.rs
│  │  │  │  ├─ index.rs
│  │  │  │  ├─ render.rs
│  │  │  │  └─ export.rs
│  │  │  └─ error/
│  │  │     ├─ mod.rs
│  │  │     └─ mapper.rs
│  ├─ plugins/
│  │  ├─ src/
│  │  │  ├─ lib.rs
│  │  │  ├─ registry/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ registry.rs
│  │  │  ├─ hook/
│  │  │  │  ├─ mod.rs
│  │  │  │  └─ hook.rs
│  │  │  └─ sandbox/
│  │  │     ├─ mod.rs
│  │  │     └─ sandbox.rs
│  └─ common/
│     ├─ src/
│     │  ├─ lib.rs
│     │  ├─ error/
│     │  │  ├─ mod.rs
│     │  │  └─ error.rs
│     │  ├─ config/
│     │  │  ├─ mod.rs
│     │  │  └─ config.rs
│     │  ├─ log/
│     │  │  ├─ mod.rs
│     │  │  └─ logger.rs
│     │  ├─ time/
│     │  │  ├─ mod.rs
│     │  │  └─ clock.rs
│     │  └─ types/
│     │     ├─ mod.rs
│     │     └─ result.rs
├─ _bmad/
│  ├─ _config/
│  ├─ bmb/
│  ├─ bmm/
│  └─ core/
├─ _bmad-output/
│  ├─ analysis/
│  └─ implementation-artifacts/
├─ .codex/
│  └─ prompts/
└─ .vscode/
```

### 3.1 core（领域模型与不变量，纯模型无 IO）

**职责**：表达领域概念与业务约束，不依赖存储/网络/系统时钟。
**设计模式**：

- core：DDD 实体/值对象/领域事件 + 策略模式（policy 校验），值对象不可变
- core::model：DDD 实体/值对象
- core::policy：策略模式 + 组合校验（validator 组合）
- core::event：领域事件 + 发布订阅（Observer/Dispatcher）
- core::error：错误代数类型（ADT）+ 映射表
  **事件使用约定**：
- services 在用例完成后发布 `DomainEvent`
- 副作用（索引/日志/缓存失效）由上层 handler 处理，core 不承载 IO

**子模块与组成**：

- core::model（实体/值对象）
  - 实体：Workspace、Folder、Document、Node
    - Workspace：id、name、folders、config_profile_id、config_override、created_at、updated_at
    - Folder：id、workspace_id、root_path、created_at、updated_at
    - UserConfig/AppConfig：全局配置（共享于多个 Workspace）
    - Document：id、folder_id、path（相对 Folder root）、title、content_hash、lang、updated_at、tree_id、ext
    - NodeBase：id、doc_id、parent_id、node_type_id、created_at、updated_at（text/range 拆到 node_text/node_range）
    - NodeType（独立 enum，变体与字段）：
      - Heading { level }
      - Text
      - Paragraph
      - BlockQuote { kind? }
      - HtmlBlock
      - List/ListItem { order, is_item }
      - CodeBlock { language }
      - CodeInline
      - Table { alignments }
      - TableHead
      - TableRow
      - TableCell
      - Image { src, alt, title }
      - Link { href, title }
      - Emphasis
      - Strong
      - Strikethrough
      - Superscript
      - Subscript
      - Task { checked }
      - FootnoteDefinition { label }
      - FootnoteReference { label }
      - DefinitionList
      - DefinitionListTitle
      - DefinitionListDefinition
      - MetadataBlock { kind }
      - MathInline
      - MathDisplay
      - HtmlInline
      - HorizontalRule
      - Wiki { target_node_id, display_text, created_at, updated_at }
    - Node（表设计拆分建议，按 NodeType 维度分表）：
      - nodes：id、doc_id、parent_id、node_type_id、created_at、updated_at
      - node_types：id、name（固定枚举，seed）
      - node_range：node_id、range_start、range_end、updated_at
      - node_text：node_id、text
      - node_heading：node_id、level
      - node_list：node_id、order、is_item
      - node_code_block：node_id、language
      - node_table：node_id、align_json
      - node_image：node_id、src、alt、title
      - node_link：node_id、href、title、link_type、ref_id
      - node_task：node_id、checked
      - node_wiki：node_id、target_node_id、display_text、created_at、updated_at
  - 值对象（建议与约束）：
    - WorkspaceId/FolderId/DocId/NodeId：强类型 ID（newtype）
    - Path：相对路径（相对 Folder root）、规范化（禁止 `..`）
    - Tag：长度与字符集约束
    - HeadingLevel：1..6
    - ContentHash：固定算法（如 SHA-256）
    - Timestamp：统一时区与格式（使用 common::time::UtcTimestamp）
  - NodeType：见上方 NodeType 变体定义
- core::policy（规则与策略）
  - 规则（示例）：层级深度限制、标题唯一性、循环引用禁止、空标题/无效标签校验、路径合法性
  - 约束分类：强约束（阻止写入）/软约束（告警）
  - 校验入口：validate_node_tree/validate_document/validate_path，提供 validate_all 组合入口
  - 作用对象：Workspace/Folder、Document、Node/NodeTree、Path/Tag/HeadingLevel 等值对象
  - 用例建议：默认内嵌在写入类用例（Create/Update/Import）；需要预检/批处理时再抽 Validate\* 用例
- core::event（领域事件）
  - 事件（示例）：
    - WorkspaceCreated：workspace_id
    - WorkspaceConfigUpdated：workspace_id、changed_keys
    - DocumentCreated：doc_id、folder_id、path
    - DocumentUpdated：doc_id、content_hash
    - DocumentDeleted：doc_id
    - TreeRebuilt：doc_id、tree_id
    - FolderAttached/FolderDetached：workspace_id、folder_id、root_path
    - NodeAdded/NodeUpdated/NodeRemoved：doc_id、node_id
- core::error（领域错误）
  - 错误（示例）：
    - NotFound：实体不存在（Workspace/Folder/Document/Node）
    - InvalidState：状态不一致（例如父节点缺失、循环引用）
    - ValidationFailed：规则校验失败（例如标题重复、路径非法）
    - Conflict：版本冲突或 hash 不匹配
    - PermissionDenied：访问被拒绝（如工作区隔离）
  - 错误编码与映射：
    - 领域错误定义稳定 code（如 `NOT_FOUND`/`INVALID_STATE`/`VALIDATION_FAILED`）
    - api::error 负责映射到 IPC/HTTP 错误与可读消息

### 3.2 services（用例编排）

应用服务/事务脚本；services::index 采用队列/调度器模式
**职责**：组合核心能力完成业务用例，管理事务边界与流程。  
**接口约定**：服务层接口建议采用 async，便于对接 sqlx 与异步任务调度。
**依赖注入约定**：repo 依赖在组合根注入（Tauri 入口或 `services::builder`），services 用例对象只接受 repo trait。
**错误映射约定**：services 统一使用 `services::error::map_domain_error` 做 DomainError → AppError 映射。
**装配结构**：Services 采用模块化子容器（Workspace/Document/Index/Config/Render/Search/Export），子容器定义在各自模块的 `mod.rs`，通过 `register(ctx, registry)` 静态方法完成装配；组合根使用 `ServicesBuilder` 装配并提供 `ServiceContext`。
**注册约定**：每个用例对象提供 `register(ctx, registry)` 静态方法（或仅需 registry 时省略 ctx），由模块 `mod.rs` 统一调用；用例对象不暴露 `new` 构造器，依赖通过 `ServiceRegistry::get<T>()` 获取。
**子模块与组成**：

- services::workspace（Workspace 生命周期）
  - CreateWorkspace：创建新 Workspace
  - AttachFolder：向 Workspace 添加根目录
  - DetachFolder：从 Workspace 移除根目录
  - SwitchWorkspace：切换当前 Workspace
  - ListWorkspace：列出已注册 Workspace
  - UpdateWorkspaceConfigOverrides：更新工作区覆盖配置
- services::config（全局配置）
  - GetGlobalConfig：获取全局配置
  - UpdateGlobalConfig：更新全局配置
  - GetEffectiveConfig：合并全局配置与工作区覆盖配置
- services::document（文档用例）
  - CreateDocument：新建文档
  - UpdateDocument：更新内容并触发增量解析
  - DeleteDocument：删除文档与关联索引
  - MoveDocument：移动/重命名
  - ScanFolder：扫描 folder 下文件，产出 DocumentSeed（path/ext/hash/title）
  - BatchImport：批量导入文件（只负责入库；按 content_hash 跳过未变更文档；删除已不存在文件）
- services::index（索引编排）
  - EnqueueParse：异步入队解析任务
  - RunParse：执行解析并写入 NodeTree/AST
  - RefreshIndex：重建或增量刷新索引（增量策略由 services::index 决定）
  - InvalidateCache：失效缓存
  - GetIndexStatus：查询解析/索引状态（待处理/进行中/失败/完成）
  - 策略：基于编辑器变更集（如 CodeMirror changes）做增量解析；优先解析当前活跃文件，后台按优先级补齐其余文件
  - 说明：services::index 负责调度与最小闭环（解析 → 收集 → 入库）；解析/索引构建由 search::parser/indexer 实现
  - 约定：services::index 提供 ParsePipeline/IndexPipeline trait；默认 ParsePipeline 使用 MarkdownParser + NodeCollectingSink
  - 用例：ReadDocument（从 FS 读取 markdown），ParseDocument（解析+收集），ApplyIndex（入库 nodes/node_text/node_range），RunIndex（执行 read→parse→apply），RunIndexNext（从队列拉取并执行）
  - 调度实现：tokio::mpsc 队列 + RunIndexWorker 后台执行；Semaphore 控制并发并对 doc_id 去重
  - 职责：接收文档变更/导入/删除事件，选择解析策略，调度队列与优先级，处理失败重试与状态上报
  - 队列协调：统一管理 ParseQueue 与 IndexQueue，按 doc_id 合并任务，顺序执行解析后索引
  - 调度策略：按优先级（活跃文档 > 最近编辑 > 其余）执行；对同一 doc_id 做去抖与合并
- services::render（渲染/预览）
  - RenderMarkdown/RenderHtml/RenderMarkmap：按格式渲染
  - RenderDocument：门面用例，按 format 路由到具体用例
  - 渲染用例按文件拆分（render/markdown.rs、render/html.rs、render/markmap.rs、render/document.rs）
- services::search（检索编排）
  - Search：全文/结构检索与分页
  - SearchScope：doc/folder/workspace
  - GetNodeTree：按 file_id 获取解析后的结构树
  - QueryHighlights：命中片段与高亮
  - GetNodeDetails：获取节点详情（含 NodeExtra）
  - GetSearchSuggestions（可选）：标签/标题建议

### 3.3 storage（持久化接口与实现）

Repository + Data Mapper（可选 Unit of Work）
（SQLite 实现）：可使用轻量 ORM 或手写 Data Mapper
**职责**：提供 Repository trait + 具体实现（fs/sqlite），不承载业务规则，仅处理持久化与序列化。  
**子模块与组成**：

- storage::repo（仓储接口）
  - WorkspaceRepository：list/get/save/delete
  - FolderRepository：list_by_workspace/get/save/delete
  - DocumentRepository：list_by_folder/get/save/delete、batch_upsert
  - NodeBaseRepository（nodes 基表）：list_by_doc/get/batch_upsert/delete_by_doc，返回 NodeBaseRow
  - Node*Repository（node*text/node_range/node\*\*）：按表读写，完整节点由 services 组装
- 说明：repo 接口建议使用 async（sqlx 驱动）
- storage::mapper（通用映射）
  - Record <-> Domain（领域映射）
  - Record 可直接 `derive(sqlx::FromRow)`，作为统一的行载体（sqlx 支持多数据库）
- storage::fs（文件系统实现）
  - 文档文件读写、目录结构管理（Markdown 原文）
- storage::sqlite（SQLite 实现）
  - 元数据与索引持久化、迁移
  - 连接管理：初始化连接/连接池与事务边界
  - SQLite 实现建议基于 `sqlx`
  - Tauri 侧通过 `tauri-plugin-sql` 管理连接池，默认使用 `plugins.sql.defaultDb` 指定的连接
  - repo 拆分为 workspace/folder/document/node 等子模块，避免单文件膨胀
    **访问边界（可见性）**：
- 对外仅暴露 `storage::repo` 与 `storage::factory`
- `storage::mapper`、`storage::fs`、`storage::sqlite` 仅 crate 内可见
  **错误映射**：
- `storage::error::map_sqlx_error` 统一将 sqlx 错误映射为 AppError
- 说明：采用 Record=FromRow 方案，不单独拆 adapter mapper
- storage::migrate（迁移与版本）
  - schema 版本、升级策略
  - sqlx migrator：迁移文件位于 `crates/storage/migrations/`，由 `ServicesBuilder` 在装配时执行
    **实现建议**：SQLite 存元数据/索引，文件系统保存 Markdown 原文。  
    **可用性策略**：快速加载元数据；索引损坏可重建；导入失败可回滚或标记异常状态。  
    **设计模式**：Repository + Data Mapper；fs/sqlite 作为 Adapter；迁移采用 Versioning；可选 Unit of Work  
    **简要说明**：
- Repository：对外暴露统一存取接口，上层不关心 FS/SQLite 细节
- Data Mapper：领域对象与表结构映射，避免持久化细节污染模型
- Adapter：fs/sqlite 实现同一接口，适配不同存储介质
- Versioning：schema 版本化管理，保障结构演进
- Unit of Work：批量操作集中提交与回滚
  **落地示例**：
- Repository：`DocumentRepository::get/save/delete/list_by_folder`
- Data Mapper：`storage::sqlite` 手写 SQL 映射领域对象 ↔ 表
- Adapter：`storage::fs`/`storage::sqlite` 适配同一仓储接口
- Versioning：`storage::migrate` 维护 schema_version 并执行迁移脚本
- Unit of Work：批量导入/解析时集中提交事务，失败回滚

### 3.4 search（索引与查询）

管道/策略模式（parser/indexer 可替换）
**职责**：解析与索引构建、检索与高亮。  
**子模块与组成**：
crates/search 骨架应体现“Parser/Indexer/Query/Schema”四层职责，且每层只保留最小可替换接口（trait + 实现 + 队列）

- search::parser（解析）
  - Markdown -> AST/NodeTree
  - 结构/文本抽取
  - 输入：markdown_text + (可选) editor_changes
  - 输出：NodeTree（高亮范围可由 node_range 生成）
  - 层级划分（严格 SRP，建议拆分）：
    - parser.rs：Parser trait + ParseTask/ParseResult/NodeTree
    - parse_queue.rs：ParseQueue（队列合并/去重）
    - mapper.rs：Event/Tag -> NodeAction（只做语义映射）
    - markdown_parser.rs：ParserState（只做栈/parent_id/range 生命周期管理）
    - TextAccumulator：只合并文本并发出 Emit（可独立为模块）
  - 实现建议：基于 pulldown-cmark 的 offset iterator 获取结构与范围
  - 输出策略：解析产出的 NodeBase/NodeType/NodeText/NodeRange 通过 NodeSink 增量推送到 indexer/service 缓冲区；parser 不聚合文本，services 可按 parent_id 聚合
  - 事件职责分离：
    - Start：确定 node_type + parent_id + start_range
    - End：补齐 end_range 并关闭节点
    - Text/Code/Html：生成 text + range（绑定当前 parent）
  - 增量（队列任务）：
    - services::index 将 editor_changes 入队（可合并/去重）
    - parser 处理单次任务，失败回退全量
    - 任务可携带优先级与重试次数
  - 类设计（示意）：
    - ParseTask { doc_id, changes, priority, retry, queued_at }
    - ParseQueue { push(task), pop(), merge_by_doc(doc_id) }
    - Parser trait { parse(task) -> ParseResult }
    - MarkdownParser (impl Parser)
    - ParseResult { node_tree, warnings }
  - 设计：Parser trait + 实现（如 MarkdownParser）
- search::indexer（索引构建）
  - 索引更新/重建、增量策略
  - 设计：Indexer trait + 实现（如 SqliteIndexer）
  - 类设计（示意）：
    - IndexTask { doc_id, node_tree, mode(full|incremental), queued_at }
    - IndexQueue { push(task), pop(), merge_by_doc(doc_id) }
    - Indexer trait { upsert(task) -> IndexResult }
    - SqliteIndexer (impl Indexer)
    - IndexResult { updated_nodes, errors }
- search::query（查询与高亮）
  - 查询解析、命中集合、片段高亮
  - 设计：Query trait + 实现（如 SqliteQuery）
  - 类设计（示意）：
    - QueryInput { q, scope(doc|folder|workspace), limit, offset }
    - Query trait { search(input) -> Hits, highlights(doc_id, q) -> Fragments }
    - SqliteQuery (impl Query)
    - Hit { node_id, path, snippet, score }
    - Fragment { node_id, ranges }
- search::schema（索引结构）
  - 索引字段定义与版本（使用 SQLite FTS）
  - FTS 字段示例：text、title_path、tags、node_type
  - schema_version 与升级规则（schema::migrate）

### 3.5 api（IPC 边界）

Facade（统一入口）
**职责**：对外命令与 DTO，参数校验与错误归一化。  
**定位说明**：`crates/api` 提供可复用的命令/DTO/错误映射与路由能力；`src-tauri/src` 仅作为 Tauri 入口适配层。Tauri 侧仅保留 `src-tauri/src/commands/mod.rs` 中的 `dispatch` 入口，命令分发由 `CommandRouter` 完成（`CommandRouter::new` 内部创建默认 CodecRegistry）。  
**子模块与组成**：

- api::command（命令定义）
  - 命令注册（CommandRegistry，仅 handler）
  - 编解码注册（CodecRegistry，仅 codec）
  - 命令路由（CommandRouter，组装 CommandPipeline）
  - Middleware（logging/codec/metrics/auth 等）
- api::dto（输入输出）
  - 请求/响应结构、序列化
- api::error（错误映射）
  - 领域错误到 IPC 错误的映射
    **类设计（示意，归属模块）**：
- api::command：ApiContext { request_id }；CommandRegistry { register() }；CodecRegistry { register() }；CommandRouter { dispatch()/with_codecs() }；PrePipeline/PostPipeline { run() }；defaults::default_router()；Router 内负责前后编解码
- api::command 示例：WorkspaceAttachFolderHandler / DocumentPingHandler
  - workspace_attach_folder：创建工作区 + 附加目录 + 扫描 + 入库 + 入队解析
- api::dto：DtoRequest/DtoResponse（envelope + payload）
- api::error：ErrorMapper { to_ipc_error(domain_error) }
  **约定**：
- 命令命名：`workspace_*`/`folder_*`/`document_*`/`search_*`/`export_*`
- request_id：由 api 生成并贯穿到 services/logs，用于链路追踪
- Tauri 侧仅注册 `dispatch(req)`，前端通过 `DtoRequest` 传入 `command` + `payload`
  **错误边界说明**：
- `common::AppError` 为内部错误模型
- Tauri 侧使用 `TauriError` 作为 IPC 适配错误（从 `api::ApiError` 转换）
  **补充说明**：
- `common::AppError` 主要用于基础设施工具（config/log/time/types）产生的错误
- 业务链路（storage/services/api）可使用各自错误类型并在边界统一映射
  **错误设计模式**：
- Anti-Corruption Layer：services/api 作为错误翻译边界，隔离内部错误变化
- Error Mapper：集中映射表（如 api::error::ErrorMapper）避免散落 match
- Error ADT：各层使用枚举错误保证穷尽与稳定错误码
  **错误码命名规范**：
- 格式：`<LAYER>_<DOMAIN>_<CODE>`（如 `CORE_WORKSPACE_NOT_FOUND`）
- 前缀建议：CORE/STORAGE/SEARCH/SERVICES/API/COMMON
- code 稳定、message 可变（可本地化），对外只承诺 code
  **IPC/对外错误结构**：
- 结构字段：`code`、`message`、`details?`、`trace_id?`
- `TauriError` 作为 IPC 错误载体，通常由 `api::ApiError` 转换
- `details` 用于调试或日志关联，前端不应依赖其稳定性
  **trace_id 注入**：
- `ApiContext` 生成并携带 `trace_id`，错误在边界统一注入
- services 层可在映射时提前注入（如 `api::error::to_api_error_with_trace`）
- 当前实现 `trace_id` 与 `request_id` 复用，必要时可拆分为独立标识

### 3.6 common（共享基础设施）

**职责**：跨模块一致性。  
**子模块与组成**：

- common::error（统一错误）
- common::config（配置加载）
- common::log（日志与追踪）
- common::time（时间戳与时区工具）
- common::types（通用类型别名）
- common::uuid（UUID/BLOB 转换）
  **类设计（示意）**：
- common::error：AppError/ErrorCode
- common::config：ConfigLoader/Config
- common::log：TraceId/LogContext
- common::time：Clock/UtcTimestamp
- common::types：AppResult<T>
- common::uuid：blob_to_uuid/uuid_to_blob（仅适用于 16 字节 UUID）
  **规范化工具（建议）**：
- PathNormalizer：统一路径分隔符、去重 `.`、阻止 `..`
- TagNormalizer：去空白、统一大小写、去重
  **实现约定（落地）**：
- Config 以 JSON 作为默认载体（FileConfigLoader 从文件读取并反序列化）
- TraceId 使用 UUID v7，LogContext 统一携带 trace_id
- Clock 统一基于 UTC（chrono::Utc）
- PathNormalizer/TagNormalizer 实现在 common::types 中，供 core/services 复用
  **模型复用约定**：
- `core::model::RelativePath` 与 `core::model::Tag` 构造需复用 normalizer
  **时间字段统一**：
- 所有 created_at/updated_at 使用 `common::time::UtcTimestamp`
- 获取当前时间必须通过 `Clock`（由 services 注入），避免在模型内直接调用系统时间
  **日志实现（推荐）**：
- 使用 `tracing` 作为日志与追踪框架
- common::log 提供 TraceId/LogContext、SpanName 及 span/info/warn/error 辅助函数（位于 crates/common/src/log.rs，推荐通过 Cargo 依赖别名 `common::` 引用）
- 具体 subscriber 初始化放在 Tauri 入口或 services 初始化层
- 测试场景可在 `crates/storage/tests/setup.rs` 通过 `enter_test_span()` 统一创建 LogContext + span，避免重复样板代码
  **职责边界（与 storage/services 协作）**：
- common::config 只负责配置“加载与解析”（如读取文本并反序列化）
- storage/services 负责配置“持久化与业务流程”（保存/更新/回滚）
- services 组合 storage 与 common：读取配置原文 → common 解析 → 构建 core::AppConfig/UserConfig

### 3.7 plugins（扩展与钩子）

Hook/Observer（事件钩子）
**职责**：受控扩展环境，统一输入输出与错误策略。  
**子模块与组成**：

- plugins::registry（插件注册）
  - 插件发现、加载、生命周期
- plugins::hook（钩子协议）
  - 解析/渲染/导出钩子接口
- plugins::sandbox（隔离与限制）
  - 资源访问与错误隔离策略
    **类设计（示意）**：
- PluginManifest { id, version, permissions, hooks }
- PluginRegistry { load(), unload(), enable(), disable(), list() }
- Plugin trait { on_parse(), on_render(), on_export() }
- Sandbox { allowlist, timeout_ms, run(plugin) }
  **钩子 DTO（示意）**：
- ParseHookInput { doc_id, markdown, changes }
- ParseHookOutput { node_tree, token_map, warnings }
- RenderHookInput { doc_id, node_tree, format, theme }
- RenderHookOutput { content }
- ExportHookInput { doc_id, node_tree, format, options }
- ExportHookOutput { artifact_path }
  **导出插件设计**：
- ExportPlugin：实现 on_export，返回导出产物路径/元数据
- ExportPipeline：负责收集插件结果并打包

### 3.8 export（导出能力）

策略 + 模板方法（不同格式共享流程）
**职责**：通过插件提供导出能力，核心仅保留导出协议与调度接口。  
**设计**：

- export 不再作为独立模块实现具体格式
- 具体格式由 plugins::hook::on_export 提供（SVG/PNG/PDF/HTML/Markdown 等）
- services::export 负责调度插件与产物打包

## 4. 依赖与层次约束

- common 不依赖其他模块
- core 依赖 common
- storage 依赖 core、common
- search 依赖 core、common
- export 依赖 core、common
- plugins 依赖 core、common
- services 依赖 core、storage、search、plugins、common
- api 依赖 services、common
- 禁止 core 依赖 m 上层模块

## 5. 核心数据流

- 导入：文件系统 -> Document/Folder 注册 -> services::index 入队解析
- 解析：search::parser 生成 AST/NodeTree，高亮范围由 node_range 派生
- 索引：search::indexer 写入 node\_\* 表与 FTS
- 查询：services::search 调用 search::query 返回命中/高亮
- 渲染：services::render 将 NodeTree 渲染为 markdown/html/markmap
- 导出：通过 plugins::hook::on_export 产出文件

---

## 6. 层级职责（SRP 理想状态）

### 6.1 core（领域模型）

- 只定义实体/值对象/领域规则与校验。
- 不包含存储、I/O、应用状态与流程编排。
- 示例：`WorkspaceId`、`Workspace`、领域校验逻辑。

### 6.2 storage（持久化层）

- 只做数据读写与映射（Repository/Mapper）。
- 不包含业务流程与用例编排。
- 示例：`WorkspaceStateRepository` 的 SQL 实现。

### 6.3 services（用例编排层）

- 只做业务流程编排，调用 repository 与基础能力（clock/index等）。
- 用例保持单一职责，按变化拆分。
- 示例：`SwitchWorkspace`、`AttachFolderAndImport`（完整流程用例）。

### 6.4 api（命令边界）

- 只做参数解析、DTO ↔ 用例调用、错误映射。
- 不承载业务流程编排。
- 示例：`WorkspaceAttachFolderHandler` 调用单个服务用例即可。

### 6.5 tauri / 前端（交互层）

- 只处理 UI/交互与状态消费。
- 不内嵌业务规则或持久化逻辑。

## 6. 扩展点

- 解析前后钩子（自定义语法）
- 渲染前后钩子（主题、节点样式）
- 导出前后钩子（模板、打包）

## 7. 可选演进

- 多用户协作：权限、审计、同步协议
- 云同步：storage 增加远端实现
- 增量更新：search 支持局部重建

---

## 8. 状态建模（以“变化”为单位）

### 8.1 单一变化：Workspace 变化

只关心与 Workspace 相关的变化：

- 当前 workspace 切换
- 最近打开 workspace
- 最近打开文件（可视为 workspace 范围内的行为）

### 8.2 对应的数据结构（建议）

`workspace_state`（只描述 workspace 相关状态）

```sql
CREATE TABLE workspace_state (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  current_workspace_id TEXT NULL,
  updated_at TEXT NOT NULL
);
```

`workspace_recent_files`（只描述 workspace 内最近文件）

```sql
CREATE TABLE workspace_recent_files (
  workspace_id TEXT NOT NULL,
  document_id TEXT NOT NULL,
  last_opened_at TEXT NOT NULL,
  position INTEGER NOT NULL,
  PRIMARY KEY (workspace_id, document_id)
);
```

### 8.3 设计原则与扩展

- `workspace_state` 只跟 workspace 变化有关。
- `workspace_recent_files` 只跟 workspace 内文件访问变化有关。
- 状态模型属于应用状态层，不进入 core 领域模型；由 services 负责用例编排，storage 负责持久化。
- 不使用 `AppState` 聚合多种变化，避免吞噬窗口/主题/权限等其他维度。
- 后续新增非 workspace 变化时，单独建表：
  - `theme_state`
  - `window_state`
  - `permission_state`
