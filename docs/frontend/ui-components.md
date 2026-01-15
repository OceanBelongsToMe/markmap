# 组件与模式

- 责任边界：可复用 UI 组件、组合式交互模式、组件约束与使用指南。
- 不负责：业务流程编排。
- 目录映射：`src/ui/components/`、`src/ui/patterns/`、`src/ui/ark/`、`src/ui/styles/`
- 交付物：组件清单、组件 API、模式库与使用示例。
- 验收指标：组件复用率、改动影响面、组件一致性合规率。
- 组件层级（建议）：
  - 基础组件：Button、Input、Select、Label、Tabs、Tooltip、IconButton
  - 复合组件：TreeView（文件树）、SearchBar（搜索栏）、Breadcrumb（路径）、TagList（标签）
  - 编辑组件：EditorToolbar、MarkdownEditor、MarkmapCanvas、EditorViewModeToggle（view mode 切换）
- 模式清单（建议）：
  - 空状态模式：无文件/无内容时引导与建议动作
  - 搜索筛选模式：左侧搜索 + 结果列表
  - 双栏编辑模式：编辑 + 预览
  - 悬浮编辑模式：markmap 预览 + 浮层编辑面板
  - 批量操作模式：多选文件/标签的批量删除、移动
- 描述模板：
  - 组件：职责 / 使用场景 / API / 交付物 / 验收指标
  - 模式：目标 / 触发条件 / 交互规则 / 交付物 / 验收指标
- 责任边界（补充）：
  - `patterns` 只负责结构与插槽，不包含业务状态与逻辑。
  - `features` 承载业务内容与交互逻辑，可调用 `src/a11y/` 工具。
  - 通用状态（如布局模式）放在 `src/state/`，不归于 `layouts` 或 `ia`。

## 渲染稳定性规范（MUST）

- 频繁更新且顺序稳定的列表/容器必须使用基础设施组件（如 `StableList`），禁止直接 `map` 生成子节点。
- 禁止在业务组件内直接使用 `Index`，仅允许在基础设施组件内部使用。
- 动态增删的列表使用 `For` 并提供稳定 key，避免节点重建与动画抖动。

## 渲染稳定性排查清单（当前项目）

- MUST（高频更新 + 顺序稳定）：`src/ui/patterns/SashContainer.tsx` 的分割线渲染需迁移到 `Index`。
- SHOULD（更新频率中等或视图抖动风险较高）：`src/features/sidebar/file-tree/FileTreeView.tsx`、`src/ui/components/TreeView.tsx`。
- OK（低频/静态）：`src/ui/components/Select.tsx`、`src/App.tsx`（routes）、`src/ui/patterns/workspace/WorkspaceSidebar.tsx`（导航模型映射）。

## 4.1 Ark UI 引入的 SRP 分层建议

- 目标：将交互状态机、视觉样式、业务语义解耦，确保每层只有一个变化理由。
- 分层边界：
  - 行为层（Ark）：只负责可访问性与状态机交互，不引入业务语义与样式。
  - 组件层（项目 API）：只暴露项目语义 props（如 `size`/`variant`），内部组合行为层并绑定样式 class。
  - 样式层（CSS）：只定义视觉规则，依赖 `data-*` 状态钩子与主题变量。
  - 业务层（features）：只使用项目组件，不直接依赖 Ark。
- 推荐目录结构：
  - `src/ui/ark/`：Ark primitives 组合层（行为层）
  - `src/ui/components/`：项目对外组件 API
  - `src/ui/styles/`：组件视觉样式
  - `src/ui/theme/`：主题 tokens 与 CSS 变量
- 变更理由映射（SRP）：
  - Ark API 变化 → 仅修改 `src/ui/ark/`
  - 视觉主题变化 → 仅修改 `src/ui/styles/` 或 `src/ui/theme/`
  - 业务语义变化 → 仅修改 `src/ui/components/` 或 `src/features/`
- Select 的层级样例：
  - 行为层：`src/ui/ark/select/Select.tsx`（组合 Ark Select primitives）
  - 组件层：`src/ui/components/Select.tsx`（封装 props + class）
  - 样式层：`src/ui/styles/select.css`（基于 `data-state`/`data-disabled`）

补充示例（当前项目）：
- EditorViewModeToggle：`src/features/workspace/components/EditorViewModeToggle.tsx`
  - 行为层：Ark ToggleGroup（仅状态机与可访问性）
  - 样式层：`src/features/workspace/components/editor-view-toggle.css`（使用 `data-part`/`data-state`）
  - 业务层：`WorkspaceToolbarContent` 仅消费组件接口

## 4.2 Ark 组件迁移清单与优先级（建议）

- 原则：先迁移交互复杂度高、可访问性风险高、复用频率高的组件。
- P0（立即收益）：
  - TreeView（当前自研递归渲染，交互与可访问性风险高）
  - Dialog/Popover/Tooltip（焦点管理与可访问性边界复杂）
  - Menu/Select/Combobox（键盘交互复杂，状态组合多）
- P1（中期收益）：
  - Tabs/Accordion/Splitter（结构性强，适合统一状态）
  - Switch/Checkbox/RadioGroup（表单一致性）
  - Slider/NumberInput/Editable（输入与状态一致性）
- P2（可选增强）：
  - Pagination/Carousel/Rating/ColorPicker/DatePicker（依赖业务价值再评估）
  - QRCode/Marquee 等展示组件（需求驱动）
- 项目映射建议（先对齐现有目录）：
  - `src/ui/components/TreeView.tsx` → Ark Tree View（P0）
  - `src/ui/components/Select.tsx` → Ark Select（P0）
  - `src/ui/components/Toolbar.tsx` → 保留自研（P2，结构稳定）
  - `src/ui/components/TextInput.tsx` → 保留自研（P1 之后再评估）
  - `src/ui/components/Button.tsx` → 保留自研（P2，样式优先）
  - `src/ui/components/useCollapsible.ts` → 可对接 Ark Collapsible（P1）

## 4.3 TreeView 组件分层草图（SRP）

- 目标：将树形交互（展开/选中/焦点/键盘）与业务语义、视觉样式分离。
- 行为层（Ark）：`src/ui/ark/tree-view/TreeView.tsx`
  - 只组合 Ark Tree View primitives，暴露稳定的 slots 与状态钩子。
  - 不引入业务语义（如文件/标签）、不包含样式。
  - 输入最小化：`items`、`selectedIds`、`expandedIds`、`on*` 事件。
- 组件层（项目 API）：`src/ui/components/TreeView.tsx`
  - 对外暴露项目语义 props（如 `variant`/`density`/`onNodeAction`）。
  - 内部组合行为层，映射业务结构到 Ark item 结构。
  - 仅绑定 class 与 slot props，不直接写样式规则。
- 样式层（CSS）：`src/ui/styles/tree-view.css`
  - 只定义视觉表现，依赖 `data-state`/`data-selected`/`data-focused`。
  - 颜色与间距只来自 `src/ui/theme/theme.css` 变量。
- 业务层（features）：`src/features/sidebar/file-tree/*`
  - 只依赖 `src/ui/components/TreeView.tsx`，不直接触碰 Ark。

示意结构（非代码）：
```
src/ui/ark/tree-view/TreeView.tsx
  - <TreeView.Root>
      <TreeView.Tree>
        <TreeView.Item>
          <TreeView.ItemText />
          <TreeView.BranchIndicator />
        </TreeView.Item>
      </TreeView.Tree>
    </TreeView.Root>

src/ui/components/TreeView.tsx
  - props: nodes, selectedId(s), onSelect, variant, density
  - map nodes -> ark items
  - class: "ui-tree-view" + variant/density modifiers

src/ui/styles/tree-view.css
  - .ui-tree-view [data-selected]
  - .ui-tree-view [data-focused]
  - .ui-tree-view [data-branch]
```

## 4.4 编辑器组件分层（CodeMirror 接入）

- 目标：将 CodeMirror 复杂 API 隔离，确保业务逻辑与 UI 渲染解耦。
- 分层架构（从下至上）：
  1.  **适配层 (Adapter)**: `src/lib/codemirror/`
      - 职责：封装第三方库，提供 `useCodeMirror` hook 与配置工厂。
      - 产出：`EditorFactory`、`extensions`、`useCodeMirror`。
      - 依赖：`@codemirror/*`，不依赖 UI 框架。
  2.  **UI 组件层 (UI)**: `src/ui/components/editor/CodeEditor.tsx`
      - 职责：纯渲染容器，处理 DOM 挂载、Resize、Props 透传。
      - 状态：非受控模式（Internal State）+ 信号同步。
      - 依赖：`useCodeMirror`。
  3.  **业务容器层 (Feature)**: `src/features/editor/MarkdownEditor.tsx`
      - 职责：连接 Store 与 UI，处理防抖保存、语言配置、业务快捷键。
      - 依赖：`CodeEditor`、`useDocument`。
  4.  **布局层 (Layout)**: `src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
      - 职责：决定编辑器的显示位置与尺寸。
      - 依赖：`MarkdownEditor`。
