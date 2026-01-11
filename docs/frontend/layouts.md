# 视觉与布局

- 责任边界：网格系统、排版层级、色彩与对比策略、视觉节奏与动效原则。
- 不负责：具体业务流程与组件逻辑。
- 目录映射：`src/layouts/`、`src/ui/theme/`、`src/ui/typography/`
- 交付物：布局规则、排版规范、主题变量（色彩/间距/字体）。
- 验收指标：可读性评分、一致性检查通过率、视觉层级清晰度。
- 结构说明：
  1. 网格与布局区域（主编辑区/侧栏/工具栏）
  2. 排版与层级（标题、正文、注释、代码）
  3. 视觉系统（色彩语义、间距、阴影、边框）

## 3.1 网格与布局区域（主编辑区/侧栏/工具栏）

- 主编辑区：页面核心区域，优先占据可用宽度，支持多列布局（编辑 + 预览/markmap）。
- 侧栏：用于文件树/标签/最近等导航信息，可折叠，默认保留最小宽度以减少干扰。
- 工具栏：承载全局操作（新建、搜索、同步、设置），位置固定，保持操作可达性。
- 布局原则：
  - 主编辑区优先级最高，侧栏与工具栏不得压缩至影响编辑可读性。
  - 侧栏可收起并记忆状态，减少上下文切换成本。
  - 工具栏与主编辑区的视觉层级明确（对比度或分隔线区分）。
  - markmap 预览优先保持可读与可操作，避免编辑器遮挡关键节点。

### 侧栏区域（职责、分区、交互）

- 职责与边界：
  - 提供全局导航与上下文切换入口（工作区/文件树/标签/最近等）。
  - 不承载编辑内容，不包含重度表单与复杂编辑交互。
  - 只负责信息入口与轻量筛选，不执行业务流程。
- 分区与内容层级：
  - 顶部：工作区切换/快速入口。
  - 中部：文件树或层级列表（主导航）。
  - 底部：标签、最近、快捷入口等辅助区。
  - 分区之间以分隔线或间距区分层级。
- 交互规则：
  - 支持折叠/展开与宽度拖拽，默认记忆上次状态。
  - 宽度变化不影响主内容可读性（最小宽度约束）。
  - 在窄屏下可退化为抽屉或图标栏，保留关键入口。
  - 列表项支持悬停与选中态强调，避免过度装饰。
- SRP 拆分建议：
  - 骨架层（layouts）：侧栏容器与宽度/折叠状态。
  - 模式层（patterns）：侧栏分区布局（顶部/中部/底部）。
  - 业务层（features）：文件树/标签/最近等具体内容。
- 骨架层设计（SideBar Skeleton）：
  - 组件：`SidebarLayer`（布局占位） + `SidebarShell`（容器边界）。
  - 状态：`collapsed`、`width`、`onToggle`、`onResize` 仅由布局/状态层提供。
  - 约束：骨架层不渲染业务内容，不处理数据流。
- 状态与样式（SRP）：
  - 状态管理：`src/state/useSidebarState.ts`（仅 state，不含 DOM/样式）。
  - 状态接口：`src/layouts/SidebarShell.tsx`（仅 props 契约与 class/style 绑定）。
  - 样式层：`src/layouts/sidebar.css`（宽度/边框/折叠态规则）。
- 约束说明（避免布局联动误伤）：
  - 侧栏骨架不修改 `AppShell` 网格结构，避免影响分割线/悬浮层定位。
  - 侧栏样式迁移到独立样式文件不应改变布局，仅影响容器视觉。
- 模式层设计（SideBar Patterns）：
  - 分区结构：`top` / `body` / `footer` 插槽。
  - `body` 为列表型内容区，可拆分多个 `Section`（文件树/最近打开/标签）。
  - 模式层只定义容器与分区，不包含业务数据或加载逻辑。
  - 组件细节与业务拆分见 `docs/frontend/features.md` 与 `docs/frontend/ui-components.md`。

### 响应式断点与布局变形规则

- 响应式断点（以骨架变形为准）：
  - 断点逻辑以“最小可用宽度”驱动，不以固定像素硬切。
  - 规则优先级：预览优先退化为浮层 → 侧栏退化为抽屉/图标栏 → 编辑区进入单列模式。
  - 规则表达：
    - 当 `W >= sideMin + editorMin + previewMin`：三列（侧栏 + 编辑 + 预览/markmap）。
    - 当 `W < sideMin + editorMin + previewMin`：预览改为悬浮/抽屉，不占布局列。
    - 当 `W < sideMin + editorMin`：侧栏折叠（抽屉或仅保留图标栏）。
    - 当 `W < editorMin`：进入单列编辑，工具栏仅保留核心动作。
  - 默认建议区间（可根据 min 值推导后调整）：
    - `W >= 1440px`：三列完整布局。
    - `1200px <= W < 1440px`：侧栏 + 编辑为主，预览切换为浮层。
    - `900px <= W < 1200px`：侧栏折叠为图标栏/抽屉。
    - `768px <= W < 900px`：工具栏精简为图标 + 更多菜单。
    - `W < 768px`：单列内容，预览仅以浮层/抽屉出现。
  - 代码模块建议（SRP）：
  - 规则层（纯函数/配置）：`src/layouts/rules/layoutBreakpoints.ts`
  - 状态层（监听尺寸 → variant）：`src/state/useResponsiveLayout.ts`
  - 视图层（消费 variant 组合骨架）：`src/layouts/MainLayout.tsx` / `src/ui/patterns/MultiPaneShell.tsx`
  - 主编辑区内容响应（SRP）：
    - 规则层：`src/layouts/rules/contentBreakpoints.ts`
    - 状态层：`src/state/useResponsiveContent.ts`
    - 视图层：`src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
  - 主编辑区内容排版规则：
    - 使用 `max-width + margin: 0 auto` 实现内容居中（极窄屏时自然占满）。

### 3.1.1 组件划分（SolidJS）

- 布局容器（骨架层）：
  - `AppShell`：布局根容器，组织三层结构与全局区域。
  - `SidebarLayer`：侧栏层，承载 `Sidebar` 区域。
  - `ContentStage`：主内容层，作为内容插槽容器（不固定内部列数）。
  - `FloatingPanelLayer`：浮层宿主，承载悬浮编辑面板插槽。
  - `ToolbarRegion`：工具栏区域（可在 `AppShell` 内部或单独区域容器）。
- 多列结构容器：
  - `MultiPaneLayout`：多列结构容器，按传入列配置渲染，不含交互逻辑。
- 区域容器：
  - 侧栏区域：文件树/标签/最近等导航区。
  - 工具栏区域：全局操作区（新建、搜索、同步、设置）。
  - 编辑区：主编辑区容器。
  - Markmap 主视图区。
  - 预览区：文档预览容器（按布局状态启用）。
- 分割线交互（通用组件）：
  - `Sash`：分割线交互元件（渲染与拖拽入口）。
  - `SashContainer`（patterns）：集中渲染多条分割线（按列计算位置）。
- 组合层（patterns）：
  - `MultiPaneShell`：组合多列结构 + 分割线交互 + 尺寸策略（不含业务逻辑）。
- 说明：骨架层仅保留 `ToolbarRegion`，`Regions.Toolbar` 不使用。
- Tauri Overlay 标题栏：
  - 使用 `titleBarStyle: "Overlay"` 时，工具栏骨架容器需加 `data-tauri-drag-region`，用于窗口拖拽。
  - 预留左右安全区（如系统按钮区）时，使用 CSS 变量控制内边距（按平台调整）。
  - macOS 右侧安全区建议值：`--titlebar-inset-right: 10px`。
- 层级关系（全链路示例）：
  - `AppShell` → `SidebarLayer` → `Sidebar`
  - `AppShell` → `ToolbarRegion` → `Toolbar`
  - `AppShell` → `ContentStage` → `MultiPaneShell` → `MultiPaneLayout` → `EditorPane` / `PreviewPane` / `MarkmapStage`
  - `AppShell` → `ContentStage` → `SashContainer` → `Sash`
  - `AppShell` → `FloatingPanelLayer` → `FloatingEditorPanel`
 
- 组件与实现细节请见 `docs/frontend/ui-components.md`。

### 3.1.2 布局状态补充：编辑 + markmap 悬浮预览

- 形态说明：markmap 作为主视图全屏展示，编辑面板以“悬浮编辑层”形式出现。
- 交互规则：
  - 编辑面板为固定悬浮层（非节点 hover 触发），可拖拽或贴边停靠。
  - 编辑面板采用避让规则，不遮挡节点文本本体，必要时自动偏移。
  - 支持最小化/收起与固定（pin）状态，减少遮挡与频繁切换。
- 尺寸与层级：
  - 悬浮编辑层占比不超过视口宽度 35%，默认半透明或带阴影区分。
  - markmap 节点优先层级最高，编辑层不得覆盖节点文本本体。

## 3.2 排版与层级（标题、正文、注释、代码）

- 层级职责：
  - 标题：结构导航与分段，引导阅读路径。
  - 正文：主要阅读内容，优先可读性与行宽控制。
  - 注释：补充信息，视觉权重降低但仍可读。
  - 代码：等宽与背景区分，避免与正文混淆。
- 规则原则：
  - 正文优先：标题不抢戏，注释弱化，代码仅在必要时突出。
  - 行宽建议：60-80 字符范围内保持稳定阅读节奏。
  - 行高建议：正文 1.5-1.7，标题略紧，注释略松。
- SRP 代码层级结构：
  - 规则层（tokens）：字号/行高/字重/颜色等基础变量。
  - 映射层（variants）：标题/正文/注释/代码与 tokens 的对应关系。
  - 视图层（Typography）：仅按 variant 选择 class，不参与计算。
  - 使用层（页面/业务）：仅选择 variant，不定义样式规则。
- 推荐目录结构：
  - `src/ui/typography/tokens.ts`
  - `src/ui/typography/variants.ts`
  - `src/ui/components/Typography.tsx`
  - `src/ui/typography/typography.css`

## 3.3 视觉系统（色彩语义、间距、阴影、边框）

- SRP 层级划分：
  - 规则层（tokens）：色彩语义/间距/阴影/圆角的基础定义。
  - 视图层（CSS 变量）：仅暴露变量，不含业务样式。
  - 使用层（组件/页面）：仅消费变量，不写硬编码色值。
- 目录结构：
  - `src/ui/theme/tokens.ts`
  - `src/ui/theme/theme.css`
- 命名建议：
  - 色彩：`--color-text-*`、`--color-bg-*`、`--color-border-*`、`--color-accent-*`
  - 间距：`--space-*`
  - 阴影：`--shadow-*`
  - 圆角：`--radius-*`
