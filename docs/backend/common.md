### 3.6 common（共享基础设施）

- 责任边界：跨模块基础设施与通用能力。
- 不负责：业务流程与领域规则。
- 目录映射：`crates/common/`。
- 交付物：配置/日志/时间/类型/错误等基础能力规范。
- 验收指标：跨模块一致性与复用度。

## 子模块与组成（高层）

- `common::error`：统一错误
- `common::config`：配置结构与解析接口
- `common::log`：日志与追踪
- `common::time`：时间戳与时区工具
- `common::types`：通用类型别名与规范化工具
- `common::uuid`：UUID/BLOB 转换

## 错误码命名规范（跨层约定）

- 格式：`<LAYER>_<DOMAIN>_<CODE>`（如 `CORE_WORKSPACE_NOT_FOUND`）
- 前缀建议：CORE/STORAGE/SEARCH/SERVICES/API/COMMON
- code 稳定、message 可变（可本地化），对外只承诺 code

## 关键约定（摘要）

- Config 以 JSON 作为默认载体（文件加载在 services::config::loader）
- TraceId 使用 UUID v7，LogContext 统一携带 trace_id
- Clock 统一基于 UTC（chrono::Utc）
- PathNormalizer/TagNormalizer 实现在 common::types 中，供 core/services 复用
- created_at/updated_at 使用 `common::time::UtcTimestamp`

## 路径索引（实现入口）

- 错误：`crates/common/src/error/`
- 配置：`crates/common/src/config/`
- 日志：`crates/common/src/log.rs`
- 时间：`crates/common/src/time/`
- 类型/规范化：`crates/common/src/types/`
- UUID：`crates/common/src/uuid.rs`

## 少量示例（方便快速理解）

- 正规化工具：`PathNormalizer` / `TagNormalizer`
- 类型别名：`AppResult<T>`
