# 任务流与交互（Frontend）

> 目的：用最少信息快速理解“前端做什么、从哪进入、如何验收”  
> 非目标：实现细节、函数/字段、组件内部逻辑

## 1) 任务流清单（只写流程与入口）

- Workspace 文件树浏览
  - 入口：`src/ui/patterns/workspace/WorkspaceSidebar.tsx`
  - 相关 UI：`src/features/sidebar/file-tree/`
  - 验收要点：文件树可展开/收起，选择节点触发打开文档

- 文档打开与渲染
  - 入口：`src/state/workspace/useActiveDocument.ts`
  - 相关 UI：`src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
  - 验收要点：选择文件后编辑区正确显示内容

- Recent 文件列表
  - 入口：`src/features/sidebar/recent-files/ui/RecentFilesSection.tsx`
  - 相关 UI：`src/features/sidebar/recent-files/`
  - 验收要点：列表展示最近打开文件，滚动触底自动加载

- 编辑器视图模式切换
  - 入口：`src/features/workspace/WorkspaceToolbarContent.tsx`
  - 相关 UI：`src/features/workspace/components/EditorViewModeToggle.tsx`
  - 验收要点：切换后 Code / Mind Map 视图正确显示

- Markmap 视图展示
  - 入口：`src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
  - 相关 UI：`src/features/markmap/MarkmapContainer.tsx`
  - 验收要点：切换到 Mind Map 后渲染节点内容

## 2) 入口索引（路径清单）

- `src/ui/patterns/workspace/WorkspaceSidebar.tsx`
- `src/features/sidebar/file-tree/`
- `src/state/workspace/useActiveDocument.ts`
- `src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
- `src/features/sidebar/recent-files/ui/RecentFilesSection.tsx`
- `src/features/sidebar/recent-files/`
- `src/features/workspace/WorkspaceToolbarContent.tsx`
- `src/features/workspace/components/EditorViewModeToggle.tsx`
- `src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
- `src/features/markmap/MarkmapContainer.tsx`

## 3) 变更边界（SRP）

- 任务流发生变化 → 更新本文件  
- 实现细节变化 → 更新对应模块文档（不要在这里写）
