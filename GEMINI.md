# Knowlattice - Gemini 项目上下文指南

## 1. 项目概览
**Knowlattice** 是一款基于 **Tauri 2** 和 **SolidJS** 构建的桌面应用程序。它以 Markmap（思维导图）为核心，提供 Markdown 知识编辑与组织功能，支持工作空间管理、文件树索引及多视图布局。

### 核心技术栈
*   **前端框架**：[SolidJS](https://www.solidjs.com/) (响应式、编译型)
*   **UI 组件库**：[Ark UI](https://ark-ui.com/) (无样式、易于访问)
*   **构建工具**：[Vite](https://vitejs.dev/)
*   **编辑器**：[CodeMirror 6](https://codemirror.net/)
*   **桌面运行时**：[Tauri 2](https://tauri.app/) (Rust)
*   **后端架构**：Rust (基于 Workspace 的多模块架构)
*   **数据库**：SQLite (通过 `sqlx` 持久化)

## 2. 环境与构建
项目推荐使用 **Bun** 作为 JavaScript 包管理器。进行桌面端开发需安装 Rust 工具链。

### 常用开发命令
| 命令 | 描述 |
| :--- | :--- |
| `bun install` | 安装前端依赖。 |
| `bun run dev` | **Web 模式**：在浏览器中运行前端（无法调用 Tauri 原生 API）。 |
| `bun tauri dev` | **Desktop 模式**：启动 Tauri 窗口及 Rust 后端。 |
| `bun run build` | 构建前端产物。 |
| `bun tauri build` | 构建最终的桌面安装包。 |

## 3. 项目结构
项目采用前后端分离且后端模块化的 monorepo 结构。

### 前端 (`src/`)
*   `src/ui/`：基于 Ark UI 的通用组件。
*   `src/state/`：全局状态管理。
*   `src/features/`：具体功能逻辑（如 `editor`, `markmap`, `workspace`）。
*   `src/layouts/`：页面布局框架（侧边栏、主区域）。

### 后端 (`src-tauri/` & `crates/`)
后端采用 Rust Workspace 组织：
*   `src-tauri/`：Tauri 应用主入口，负责窗口创建与命令分发。
*   `crates/`：解耦的后端领域模块。
    *   `crates/api`：Tauri Command 命令层与 DTO。
    *   `crates/services`：业务用例与核心逻辑。
    *   `crates/storage`：数据库访问（SQLite）与迁移。
    *   `crates/search`：解析、索引与全文检索（FTS）。
    *   `crates/core`：核心领域模型与常量。

### 本地包 (`packages/`)
*   `packages/markmap-common` & `markmap-view`：Markmap 的本地定制版本。

## 4. 关键架构与数据流
*   **数据流向**：导入 → 解析 → 索引 → 查询 → 渲染 → 导出。
*   **渲染稳定性**：频繁更新的列表必须使用 `StableList` 组件，禁止直接在复杂业务中使用 `Index`。
*   **SRP 边界**：严格遵守分层原则，`api` 负责契约，`services` 负责逻辑，`storage` 负责持久化。

## 5. 重要文档索引
*   `project-context.md`：项目的全貌入口与关键索引。
*   `docs/architecture-summary.md`：详细的架构摘要与组件拓扑。
*   `README.md`：项目基础运行说明。