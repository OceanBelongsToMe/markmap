# 任务流与交互

- 责任边界：关键任务链路、状态流转、错误与恢复策略、交互反馈规则。
- 不负责：视觉主题与组件样式。
- 目录映射：`src/features/`（内聚在具体特性模块中）
- 交付物：任务流图、状态机/流程编排、错误处理与恢复规则。
- 验收指标：任务完成率、错误率、恢复成功率。

## Workspace 文件树前端拆分（SRP）

### 模块拆分

- API 适配层：`src/features/workspace/api/workspaceApi.ts`
- 状态层：`src/state/workspace/useWorkspaceTree.ts`
- 行为层：`src/features/workspace/hooks/useWorkspaceTreeActions.ts`
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
- 事件层：`src/state/workspace/useDocumentEvents.ts`
- 渲染资源：`src/features/document/hooks/useDocumentRender.ts`
- 交互层：`src/features/sidebar/file-tree/`
  - 监听节点选中事件，调用 `openDocument`。
- 视图层：`src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
  - 布局与组件细节见 `docs/frontend/layouts.md` 与 `docs/frontend/ui-components.md`。

## Recent 文件列表（SRP）

### 模块拆分

- API 适配层：`src/features/workspace/api/workspaceApi.ts`
  - `workspace_recent_files_list` / `workspace_recent_file_record`
- 状态层：`src/state/workspace/useRecentFiles.ts`
- 视图模型层：`src/features/sidebar/recent-files/data/*`
  - recent + fileTree 映射为扁平 `FileTreeNode[]`
  - 按 “今天 / 过去7天 / 更早” 分组
- 视图层：`src/features/sidebar/recent-files/ui/RecentFilesSection.tsx`
  - 与 Files 平级展示，空列表不渲染

### 交互与职责边界

- Recent 订阅 `document-open` 事件并乐观置顶，同时异步记录最近打开。
- Recent 只展示文件节点（扁平列表），不展示树结构。
- 列表触底自动加载，分页由状态层控制。
