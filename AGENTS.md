# Repository Guidelines

## 项目结构与模块组织

- `src/`：SolidJS 前端入口与页面，功能模块在 `src/features/`，通用 UI 在 `src/ui/`。
- `public/`：Vite 静态资源目录。
- `src-tauri/`：Tauri 应用入口与配置（`src-tauri/tauri.conf.json`）。
- `crates/`：Rust workspace 业务与基础设施模块（如 `crates/api`、`crates/services`、`crates/storage`）。
- `packages/`：前端共享 workspace 包（如 markmap 相关包）。
- `docs/`：设计、架构与流程文档（入口：`docs/index.md`）。

## 构建、测试与本地开发命令

- `bun install`：安装前端依赖与 workspace 包。
- `bun run build`：构建前端产物。
- `bun tauri dev`：启动桌面端（Tauri + Rust 后端）。
- `cargo test`：运行 Rust 全部测试。
- `cargo test -p <crate>`：运行指定 crate（示例：`cargo test -p storage`）。

## 编码风格与命名约定

- TypeScript/TSX：2 空格缩进，双引号字符串，必要时显式类型增强可读性。
- Rust：遵循 `rustfmt` 默认格式；函数/模块用 `snake_case`，类型用 `CamelCase`。
- 组织方式：优先按功能拆分到 `src/features/<feature>/...`，通用 UI 组件放在 `src/ui/`。

## 测试规范

- 后端测试集中在 `crates/*/tests` 与 `#[cfg(test)]` 模块中。
- 使用 `#[test]` 或 `#[tokio::test]` 与现有测试风格保持一致。
- 当前未配置前端测试框架；如需新增，请同时补齐工具链与脚本。

## 提交与 PR 规范

- 提交消息采用 Conventional Commits（示例：`feat(markmap): unify inline editable text`、`chore: ignore workspace artifacts`）。
- PR 需包含简要说明、测试命令与结果；涉及 UI 变更时附截图/GIF。

## 文档与架构入口

- 项目全貌：`project-context.md`。
- 文档导航：`docs/index.md`。
- 架构摘要：`docs/architecture-summary.md`。
- 历史设计归档：`docs/_archive/module-design.md`、`docs/_archive/ux-ui-design.md`。
