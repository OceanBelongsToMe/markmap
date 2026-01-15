# Project Context

- 责任边界：项目全貌入口与关键索引。
- 不负责：模块内部实现细节。
- 目录映射：`project-context.md`。
- 交付物：项目全貌说明与关键入口链接。
- 验收指标：2 分钟内理解项目全貌。

## 1) 项目定位

基于 Tauri + Rust 的 Markdown/Markmap 编辑与检索系统，强调可扩展后端与清晰职责边界。

## 2) 代码结构总览

- Frontend：`src/`
- Tauri 入口：`src-tauri/src/`
- Rust workspace：`crates/`

## 3) 文档入口

- 文档导航：`docs/index.md`
- 架构摘要：`docs/architecture-summary.md`
- AI 协作协议：`AGENTS.md`

## 4) 关键模块索引

- Frontend 概览：`docs/frontend/overview.md`
- Backend 概览：`docs/backend/overview.md`
- Shared 概览：`docs/shared/overview.md`
- Process 概览：`docs/process/overview.md`

## 5) 构建与运行（常用）

- 前端开发：`bun run dev`
- Tauri 开发：`bun tauri dev`

## 6) 文档维护规则（IMPORT！！！）

- 过程与记录入口：`docs/process/overview.md`
