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

export const WorkspacePage = () => {
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
  const showPreview = () => layoutMode() === "split" && layoutVariant() === "three-pane";
  const showSidebar = () => layoutVariant() !== "single-pane";

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
            />
          }
        />
      }
      content={
        <div
          ref={(el) => setShellRef(el)}
          style={{ width: "100%", height: "100%", position: "relative" }}
        >
          <WorkspaceSplitShell
            sidebar={
              showSidebar() ? (
                <WorkspaceSidebar
                  collapsed={collapsed()}
                  onToggle={() => setCollapsed(!collapsed())}
                  fileTreeStyle={fileTreeStyle()}
                />
              ) : undefined
            }
            sidebarWidth={sidebarWidth()}
            editor={<WorkspaceEditorPane viewMode={viewMode()} />}
            preview={showPreview() ? <WorkspacePreviewPane /> : undefined}
            onSizesChange={(sizes) => {
              if (!showSidebar()) return;
              const next = sizes[0];
              if (typeof next === "number") {
                setSidebarWidth(next);
              }
            }}
          />
        </div>
      }
      floatingPanel={
        <FloatingEditorPanelContent />
      }
    />
  );
};
