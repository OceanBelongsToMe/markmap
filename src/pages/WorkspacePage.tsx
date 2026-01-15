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
import { Sash } from "../layouts/Sash";
import type { FileTreeStyle } from "../features/sidebar/file-tree";

export const WorkspacePage = () => {
  // ... (状态保持不变)
  const { layoutMode } = useLayoutState();
  const {
    collapsed,
    setCollapsed,
    width: sidebarWidth,
    setWidth: setSidebarWidth
  } = useSidebarState();
  const [shellRef, setShellRef] = createSignal<HTMLDivElement | undefined>();
  const { layoutVariant } = useResponsiveLayout(
    () => shellRef(),
    workspaceLayoutMins
  );
  const [fileTreeStyle, setFileTreeStyle] = createSignal<FileTreeStyle>("ark");
  const [viewMode, setViewMode] = createSignal<"code" | "markmap">("code");
  const [isHoveringSidebar, setIsHoveringSidebar] = createSignal(false);

  const showPreview = () => layoutMode() === "split" && layoutVariant() === "three-pane";
  const showSidebar = () => layoutVariant() !== "single-pane";

  const isSidebarFixed = () => !collapsed() && showSidebar();
  const isOverlayVisible = () => collapsed() && isHoveringSidebar();

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
                if (!next) setIsHoveringSidebar(false);
              }}
            />
          }
        />
      }
      content={
        <div style={{ width: "100%", height: "100%", position: "relative" }}>
          {collapsed() && (
            <div
              class="sidebar-trigger"
              onMouseEnter={() => setIsHoveringSidebar(true)}
            />
          )}

          {/* Layer 2: 悬停 Overlay */}
          <div
            class="sidebar-overlay-container"
            classList={{ "is-visible": isOverlayVisible() }}
            style={{ width: `${sidebarWidth()}px` }}
            onMouseLeave={() => setIsHoveringSidebar(false)}
          >
            {collapsed() && (
              <>
                <WorkspaceSidebar
                  collapsed={false}
                  isOverlay={true}
                  fileTreeStyle={fileTreeStyle()}
                />
                <Show when={isOverlayVisible()}>
                  <Sash
                    left={sidebarWidth()}
                    onDrag={(x) => setSidebarWidth(x)}
                  />
                </Show>
              </>
            )}
          </div>

          <div
            ref={(el) => setShellRef(el)}
            style={{ width: "100%", height: "100%", position: "relative", overflow: "hidden" }}
          >
            <WorkspaceSplitShell
              sidebar={
                isSidebarFixed() ? (
                  <WorkspaceSidebar
                    collapsed={false}
                    fileTreeStyle={fileTreeStyle()}
                  />
                ) : undefined
              }
              sidebarWidth={sidebarWidth()}
              editor={<WorkspaceEditorPane viewMode={viewMode()} />}
              preview={showPreview() ? <WorkspacePreviewPane /> : undefined}
              onSizesChange={(sizes) => {
                if (isSidebarFixed()) {
                  const next = sizes[0];
                  if (typeof next === "number") {
                    setSidebarWidth(next);
                  }
                }
              }}
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