# 任务流与交互

- 责任边界：关键任务链路、状态流转、错误与恢复策略、交互反馈规则。
- 不负责：视觉主题与组件样式。
- 目录映射：`src/features/`（内聚在具体特性模块中）
- 交付物：任务流图、状态机/流程编排、错误处理与恢复规则。
- 验收指标：任务完成率、错误率、恢复成功率。

## Workspace 文件树前端拆分（SRP）

### 模块拆分

- API 适配层：`src/features/workspace/api/workspaceApi.ts`
  - `getCurrentWorkspace()` / `getWorkspaceFileTree(workspaceId)`
- 状态层：`src/state/workspace/useWorkspaceTree.ts`
  - `currentWorkspace` / `tree` / `loading` / `error`
- 行为层：`src/features/workspace/hooks/useWorkspaceTreeActions.ts`
  - `loadCurrentWorkspace()` / `refreshTree()`
- 视图层：`src/ui/patterns/workspace/WorkspaceSidebar.tsx`
  - 只负责渲染树，不负责数据请求

### 交互与职责边界

- `WorkspaceSidebar` 消费状态，不执行 `invoke`。
- API 层只做命令调用与 DTO 映射，不包含业务逻辑。
- 状态层负责缓存与刷新策略，避免 UI 组件重复请求。

## 文档打开流程（SRP）

### 模块拆分

- API 适配层：`src/features/workspace/api/workspaceApi.ts`
  - `renderDocument(docId, format)`: 调用后端命令获取渲染内容。
- 状态层：`src/state/workspace/useActiveDocument.ts`
  - `activeDocId`: 当前激活的文档 ID。
  - `documentResource`: 基于 activeDocId 的异步资源（content/loading/error）。
  - `openDocument(id)`: 动作入口。
- 交互层：`src/features/sidebar/file-tree/`
  - 监听节点选中事件，调用 `openDocument`。
- 视图层：`src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
  - 消费 `documentResource`，根据 loading/error 状态渲染骨架屏或 `MarkdownEditor`。
  - 布局与组件细节见 `docs/frontend/layouts.md` 与 `docs/frontend/ui-components.md`。
