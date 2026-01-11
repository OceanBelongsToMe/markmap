### 3.1 core（领域模型与不变量，纯模型无 IO）

- 责任边界：领域概念与业务不变量表达。
- 不负责：存储、I/O、流程编排与系统时钟。
- 目录映射：`crates/core/`。
- 交付物：实体/值对象/领域事件/领域错误。
- 验收指标：不变量可验证性、领域模型稳定性。

## 设计原则

- core 仅表达领域概念与业务约束，不依赖存储/网络/系统时钟。
- 不暴露 Repository/Service/Indexer/Clock 等操作接口。
- 仅返回 `Result<T, DomainError>`，错误映射由上层处理。

## 模块构成（高层）

- `core::model`：实体/值对象
- `core::policy`：规则与校验策略
- `core::event`：领域事件（结构定义）
- `core::error`：领域错误

## 关键约定（摘要）

- 事件由 services 在用例完成后发布；副作用由上层 handler 处理。
- Path/Tag 等值对象使用 common 的规范化工具。
- created_at/updated_at 使用 `common::time::UtcTimestamp`。

## 路径索引（实现入口）

- 实体/值对象：`crates/core/src/model/`
- 规则校验：`crates/core/src/policy/`
- 领域事件：`crates/core/src/event/`
- 领域错误：`crates/core/src/error/`

## 少量示例（方便快速理解）

- 实体：Workspace / Folder / Document / Node
- 值对象：WorkspaceId / FolderId / DocId / NodeId
- 校验入口：validate_node_tree / validate_document / validate_path
