# 代码对齐清单（文档模块映射）

> 用于将文档拆分方案映射到实际代码入口与核心文件，便于连续使用编程助手时快速定位。

## A) Frontend（docs则：frontend/*）

### information-architecture.md
- `src/ia/nav-model.ts`
- `src/ia/taxonomy.ts`
- `src/ia/labels.ts`

### i18n.md
- `src/i18n/index.ts`
- `src/i18n/context.tsx`
- `src/i18n/locale.ts`
- `src/i18n/supported-locales.ts`
- `src/i18n/formatters.ts`
- `src/i18n/zh-CN.ts`

### layouts.md
- `src/layouts/AppShell.tsx`
- `src/layouts/MainLayout.tsx`
- `src/layouts/MultiPaneLayout.tsx`
- `src/layouts/SidebarShell.tsx`
- `src/layouts/Regions.tsx`
- `src/layouts/Layers.tsx`
- `src/layouts/Sash.tsx`
- `src/layouts/sidebar.css`
- `src/layouts/rules/layoutBreakpoints.ts`
- `src/layouts/rules/contentBreakpoints.ts`
- `src/layouts/rules/workspaceLayoutSizes.ts`

### ui-components.md
- `src/ui/components/Button.tsx`
- `src/ui/components/Select.tsx`
- `src/ui/components/TextInput.tsx`
- `src/ui/components/Toolbar.tsx`
- `src/ui/components/StableList.tsx`
- `src/ui/components/Typography.tsx`
- `src/ui/components/TreeView.tsx`
- `src/ui/components/useCollapsible.ts`
- `src/ui/components/collapsible.css`
- `src/ui/components/markmap/MarkmapCanvas.tsx`
- `src/ui/components/markmap/markmap.css`

### ui-components.md（patterns 扩展）
- `src/ui/patterns/MultiPaneShell.tsx`
- `src/ui/patterns/ToolbarShell.tsx`
- `src/ui/patterns/SashContainer.tsx`
- `src/ui/patterns/sidebar/SidebarSection.tsx`
- `src/ui/patterns/workspace/WorkspaceSidebar.tsx`
- `src/ui/patterns/workspace/WorkspaceEditorPane.tsx`
- `src/ui/patterns/workspace/WorkspacePreviewPane.tsx`
- `src/ui/patterns/workspace/WorkspaceSplitShell.tsx`
- `src/ui/patterns/workspace/WorkspaceFloatingPanel.tsx`

### state-management.md
- `src/state/useSidebarState.ts`
- `src/state/useLayoutState.ts`
- `src/state/usePaneSizes.ts`
- `src/state/useMeasuredWidth.ts`
- `src/state/useResponsiveLayout.ts`
- `src/state/useResponsiveContent.ts`
- `src/state/window/index.ts`
- `src/state/window/fullscreen.ts`
- `src/state/workspace/useActiveDocument.ts`
- `src/state/workspace/useWorkspaceTree.ts`

### features.md
- `src/features/workspace/WorkspacePanel.tsx`
- `src/features/workspace/WorkspacePanelView.tsx`
- `src/features/workspace/WorkspaceToolbarContent.tsx`
- `src/features/workspace/EditorSettings.tsx`
- `src/features/workspace/components/EditorSettingsView.tsx`
- `src/features/workspace/components/FloatingEditorPanelView.tsx`
- `src/features/workspace/FloatingEditorPanelContent.tsx`
- `src/features/workspace/hooks/useWorkspaceActions.ts`
- `src/features/workspace/hooks/useWorkspaceTreeActions.ts`
- `src/features/workspace/hooks/useWorkspaceTree.ts`
- `src/features/workspace/hooks/useEditorSettings.ts`
- `src/features/workspace/hooks/useFloatingEditorPanel.ts`
- `src/features/workspace/api/workspaceApi.ts`
- `src/features/editor/MarkdownEditor.tsx`
- `src/features/document/hooks/useDocumentRender.ts`
- `src/features/markmap/MarkmapContainer.tsx`

### sidebar-file-tree（features 子模块示例）
- `src/features/sidebar/file-tree/index.ts`
- `src/features/sidebar/file-tree/FileTreeSection.tsx`
- `src/features/sidebar/file-tree/domain/*`
- `src/features/sidebar/file-tree/data/*`
- `src/features/sidebar/file-tree/state/*`
- `src/features/sidebar/file-tree/ui/*`
- `src/features/sidebar/file-tree/styles/*`

### accessibility.md
- `src/a11y/focus/index.ts`
- `src/a11y/focus/trap.ts`
- `src/a11y/focus/restore.ts`

### 入口与路由（文档导航关联）
- `src/index.tsx`
- `src/App.tsx`
- `src/routes/index.ts`
- `src/routes/workspace.ts`
- `src/routes/types.ts`
- `src/pages/WorkspacePage.tsx`

## B) Backend（docs则：backend/*）

### tauri-entry.md
- `src-tauri/src/main.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/app/mod.rs`
- `src-tauri/src/app/db.rs`
- `src-tauri/src/app/window_events/mod.rs`
- `src-tauri/src/app/window_events/fullscreen.rs`
- `src-tauri/src/commands/mod.rs`
- `src-tauri/src/error.rs`

### api-dto.md
- `crates/api/src/dto/mod.rs`
- `crates/api/src/dto/workspace.rs`
- `crates/api/src/dto/folder.rs`
- `crates/api/src/dto/document.rs`
- `crates/api/src/dto/index.rs`
- `crates/api/src/dto/search.rs`
- `crates/api/src/dto/render.rs`
- `crates/api/src/dto/export.rs`
- `src-tauri/src/dto/mod.rs`

### error-model.md
- `crates/api/src/error/mod.rs`
- `crates/api/src/error/mapper.rs`
- `crates/core/src/error/mod.rs`
- `crates/core/src/error/domain_error.rs`
- `crates/common/src/error/mod.rs`
- `crates/common/src/error/error.rs`
- `crates/services/src/error.rs`
- `crates/storage/src/error.rs`
- `crates/search/src/error.rs`

### core-domain.md
- `crates/core/src/lib.rs`
- `crates/core/src/model/*`
- `crates/core/src/policy/*`
- `crates/core/src/event/*`

### services.md
- `crates/services/src/lib.rs`
- `crates/services/src/builder.rs`
- `crates/services/src/workspace/*`
- `crates/services/src/document/*`
- `crates/services/src/index/*`
- `crates/services/src/search/*`
- `crates/services/src/render/*`
- `crates/services/src/export/*`
- `crates/services/src/config/*`

### storage.md
- `crates/storage/src/lib.rs`
- `crates/storage/src/provider.rs`
- `crates/storage/src/factory.rs`
- `crates/storage/src/repo/*`
- `crates/storage/src/mapper/*`
- `crates/storage/src/sqlite/*`
- `crates/storage/src/fs/*`
- `crates/storage/migrations/*.sql`

### search.md
- `crates/search/src/lib.rs`
- `crates/search/src/domain/*`
- `crates/search/src/application/*`
- `crates/search/src/adapters/*`

### export.md
- `crates/export/src/lib.rs`
- `crates/export/src/pipeline/*`
- `crates/export/src/format/*`
- `crates/export/src/theme/*`

### plugins.md（如需要）
- `crates/plugins/src/lib.rs`
- `crates/plugins/src/registry/*`
- `crates/plugins/src/hook/*`
- `crates/plugins/src/sandbox/*`

## C) Shared（docs则：shared/*）

### markmap-protocol.md
- `crates/services/src/render/markmap/pipeline/transformer.rs`
- `crates/services/src/render/markmap/pipeline/initializer.rs`
- `crates/services/src/render/markmap/pipeline/folder.rs`
- `crates/services/src/render/markmap/config/options.rs`
- `crates/services/src/render/markmap/types.rs`
- `crates/services/src/render/markmap/config/provider.rs`
- `src/features/markmap/MarkmapContainer.tsx`
- `src/ui/components/markmap/MarkmapCanvas.tsx`
- `src/features/document/hooks/useDocumentRender.ts`

### config-scopes.md
- `crates/storage/src/repo/user_settings_repo.rs`
- `crates/storage/src/sqlite/sql/user_settings.rs`
- `crates/storage/src/mapper/user_settings.rs`
- `crates/services/src/config/*`

### naming-conventions.md
- `src/ia/taxonomy.ts`
- `src/ia/labels.ts`
- `crates/common/src/types/*`
- `crates/common/src/uuid.rs`

## D) Process（docs则：process/*）

### task-log.md
- `docs/task.md`（迁移内容，不涉及代码）

### decisions.md
- `docs/process/decisions.md`
