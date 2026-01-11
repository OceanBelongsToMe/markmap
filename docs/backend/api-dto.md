### 3.5 api（IPC 边界）

- 责任边界：命令与 DTO 边界、参数校验与错误映射入口。
- 不负责：业务流程编排与领域规则。
- 目录映射：`crates/api/`、`src-tauri/src/commands/`、`src-tauri/src/dto/`。
- 交付物：命令定义、DTO 结构、路由与分发规则。
- 验收指标：接口稳定性、边界清晰度。

## 架构概览

- `crates/api` 提供可复用的命令/DTO/错误映射与路由能力。
- `src-tauri/src` 仅作为 Tauri 入口适配层。
- 命令分发由 `default_router` 或 `CommandRouter::with_codecs` 完成。

## 子模块与组成（高层）

- `api::command`：命令定义与注册（CommandRegistry/CodecRegistry）
- `api::dto`：请求/响应结构与序列化
- `api::error`：领域错误到 IPC 错误的映射
- `CommandRouter`：基于 registry + codecs 组装管线

## 约定

- 命令命名：`workspace_*` / `folder_*` / `document_*` / `search_*` / `export_*`
- `request_id` 由 api 生成并贯穿到 services/logs
- 错误码规范与结构约定见 `docs/backend/common.md`

## 路径索引（实现入口）

- 命令与路由：`crates/api/src/dispatch/`
- DTO 定义：`crates/api/src/dto/`
- 错误映射：`crates/api/src/error/`
- Tauri 入口：`src-tauri/src/commands/mod.rs`、`src-tauri/src/dto/mod.rs`

## 少量示例（方便快速理解）

- 路由注册：`default_router()` / `default_registry()`
- 请求/响应结构：`DtoRequest` / `DtoResponse`
