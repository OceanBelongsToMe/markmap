import { createSignal } from "solid-js";
import { MainLayout } from "../layouts/MainLayout";
import { workspaceLayoutMins } from "../layouts/rules/workspaceLayoutSizes";
import { useLayoutState } from "../state/useLayoutState";
import { useSidebarState } from "../state/useSidebarState";
import { useResponsiveLayout } from "../state/useResponsiveLayout";
import { FloatingEditorPanelContent } from "../features/workspace/FloatingEditorPanelContent";
import { WorkspaceToolbarContent } from "../features/workspace/WorkspaceToolbarContent";
import { WorkspaceEditorPane } from "../ui/patterns/workspace/WorkspaceEditorPane";
import { WorkspacePreviewPane } from "../ui/patterns/workspace/WorkspacePreviewPane";
import { WorkspaceSidebar } from "../ui/patterns/workspace/WorkspaceSidebar";
import { WorkspaceSplitShell } from "../ui/patterns/workspace/WorkspaceSplitShell";
import { ToolbarShell } from "../ui/patterns/ToolbarShell";
import type { FileTreeStyle } from "../features/sidebar/file-tree";
import { useWorkspacePageOrchestrator } from "../features/workspace/hooks/useWorkspacePageOrchestrator";
import { UnifiedSidebarShell, type SidebarMode } from "../layouts/UnifiedSidebarShell";
import { useSidebarInteraction } from "../features/workspace/hooks/useSidebarInteraction";

export const WorkspacePage = () => {
  const { layoutMode } = useLayoutState();
  const {
    collapsed,
    setCollapsed,
    width: sidebarWidth,
    setWidth: setSidebarWidth
  } = useSidebarState();
  
  useWorkspacePageOrchestrator();
  
  const [shellRef, setShellRef] = createSignal<HTMLDivElement | undefined>();
  const { layoutVariant } = useResponsiveLayout(
    () => shellRef(),
    workspaceLayoutMins
  );
  
  const [fileTreeStyle, setFileTreeStyle] = createSignal<FileTreeStyle>("ark");
  const [viewMode, setViewMode] = createSignal<"code" | "markmap">("code");
  
  // 使用封装的交互 Hook
  const { isHovering: isHoveringSidebar, handlers: sidebarHandlers } = useSidebarInteraction(collapsed);

  const showPreview = () => layoutMode() === "split" && layoutVariant() === "three-pane";
  const showSidebar = () => layoutVariant() !== "single-pane";

  const sidebarMode = (): SidebarMode => {
    if (!showSidebar()) return "hidden";
    return collapsed() ? "overlay" : "fixed";
  };

  const isOverlayVisible = () => sidebarMode() === "overlay" && isHoveringSidebar();

  return (
    <MainLayout
      toolbar={
        <ToolbarShell
          left={
            <WorkspaceToolbarContent
              fileTreeStyle={fileTreeStyle()}
              onFileTreeStyleChange={setFileTreeStyle}
              viewMode={viewMode()}
              onViewModeChange={setViewMode}
              sidebarCollapsed={collapsed()}
              onToggleSidebar={() => {
                const next = !collapsed();
                setCollapsed(next);
                if (!next) sidebarHandlers.reset();
              }}
            />
          }
        />
      }
      content={
        <div 
          ref={(el) => setShellRef(el)}
          style={{ width: "100%", height: "100%", position: "relative", display: "flex" }}
        >
          {/* 触发器：仅在折叠且非悬停时显示 */}
          {collapsed() && (
            <div
              class="sidebar-trigger"
              onMouseEnter={sidebarHandlers.onMouseEnter}
            />
          )}

          {/* 统一侧边栏容器 */}
          <UnifiedSidebarShell
            mode={sidebarMode()}
            width={sidebarWidth()}
            onWidthChange={setSidebarWidth}
            classList={{ "is-visible": isOverlayVisible() }}
            sashVisible={sidebarMode() === "fixed" || isOverlayVisible()}
            onDragStart={sidebarHandlers.onDragStart}
            onDragEnd={sidebarHandlers.onDragEnd}
            onMouseEnter={sidebarHandlers.onMouseEnter}
            onMouseLeave={sidebarHandlers.onMouseLeave}
          >
            <WorkspaceSidebar
              collapsed={false} // 始终显示内容，因为 UnifiedSidebarShell 处理形态
              fileTreeStyle={fileTreeStyle()}
            />
          </UnifiedSidebarShell>

          {/* 主内容区域 */}
          <div style={{ flex: 1, "min-width": 0, position: "relative", overflow: "hidden" }}>
            <WorkspaceSplitShell
              editor={<WorkspaceEditorPane viewMode={viewMode()} />}
              preview={showPreview() ? <WorkspacePreviewPane /> : undefined}
            />
          </div>
        </div>
      }
      floatingPanel={
        <FloatingEditorPanelContent />
      }
    />
  );
};
