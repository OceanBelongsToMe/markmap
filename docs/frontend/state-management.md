# 状态管理（State）

- 责任边界：全局/共享状态的定义、存取与生命周期管理。
- 不负责：UI 视图渲染、布局结构定义、业务流程编排。
- 目录映射：`src/state/`
- 交付物：状态模型定义、状态读写接口、状态更新策略。
- 验收指标：状态变更可追踪性、状态同步一致性、跨组件复用度。

## 模块划分（与现有目录对齐）

### 1) 布局与视口状态

- `src/state/useLayoutState.ts`：布局状态聚合（当前布局形态/面板组合）。
- `src/state/usePaneSizes.ts`：分栏尺寸与持久化策略。
- `src/state/useMeasuredWidth.ts`：容器宽度测量与派生状态。
- `src/state/useResponsiveLayout.ts`：断点驱动的布局变体。
- `src/state/useResponsiveContent.ts`：内容区响应式状态（内容布局变体）。

### 2) 侧栏状态

- `src/state/useSidebarState.ts`：侧栏折叠、宽度与交互状态。

### 3) 窗口状态

- `src/state/window/index.ts`
- `src/state/window/fullscreen.ts`

### 4) 工作区状态

- `src/state/workspace/useActiveDocument.ts`：当前打开文档与资源状态。
- `src/state/workspace/useWorkspaceTree.ts`：工作区文件树状态与资源。

### 5) Markmap 懒加载状态

- 仅维护“加载状态/已加载标记”，不负责数据请求。
- 数据请求与加载策略放在 `features/*` 的 hooks（如 `useMarkmapTreeActions`）。
- 若需要跨组件共享加载进度，可在 `src/state/` 定义轻量状态（如 `children_loaded`、`loading`）。

## SRP 约束

- 状态层只管理 state 与派生 state，不直接执行数据请求或命令调用。
- 数据请求与业务动作放在 `features/*` 的 hooks（如 `useWorkspaceTreeActions`）。
- 视图层（layouts/ui/components）只消费状态，不修改状态结构。
- 跨层共享规则：状态变化触发 UI 更新，但不反向决定业务流程。

## 依赖关系

- `state` 可依赖 `lib` 的纯工具与类型。
- `state` 不依赖 `layouts`、`ui`、`features`，避免循环依赖。
- `features` 依赖 `state` 获取状态与更新入口。
