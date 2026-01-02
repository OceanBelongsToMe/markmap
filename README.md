# Knowlattice

Tauri + SolidJS 桌面应用：以 Markmap 为核心的 Markdown 知识编辑与组织工具，支持工作空间、文件树与多视图布局。

## 功能概览

- 工作空间与最近文件状态（SQLite 持久化）
- 侧栏文件树、编辑区、Markmap 预览的多列布局
- Tauri 2 + Rust 后端服务分层（API / Services / Storage）

## 开发环境

- Node.js（或 Bun）
- Rust toolchain（Tauri 2）
- 平台依赖见 Tauri 文档

## 运行方式

### Web 模式（仅前端）

```bash
npm install
npm run dev
```

### Desktop 模式（Tauri）

```bash
npm install
npm run tauri dev
```

## 项目结构

- `src/` 前端（SolidJS）
  - `src/ui/components` 通用组件（含渲染基础设施）
  - `src/ui/patterns` 组合型 UI 模式
  - `src/state` 前端状态
- `src-tauri/` Tauri 入口（Rust）
- `crates/` 后端领域与服务拆分
  - `crates/api` 命令层与 DTO
  - `crates/services` 用例与业务逻辑
  - `crates/storage` SQLite 访问与迁移
- `docs/` 设计与模块文档

## 设计文档

- `docs/module-design.md`：后端模块与 SRP 结构
- `docs/ux-ui-design.md`：布局与交互规范

## 渲染稳定性规范（摘要）

- 频繁更新且顺序稳定的列表必须使用 `StableList`
- 禁止业务组件直接使用 `Index`

## License

Proprietary. All rights reserved.
No public redistribution, modification, or use without explicit written permission.
