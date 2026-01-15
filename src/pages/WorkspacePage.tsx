import { createSignal, Show } from "solid-js";
import { MainLayout } from "../layouts/MainLayout";
// ... (其他 import 保持不变)

export const WorkspacePage = () => {
  // ... (状态定义保持不变)

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
        <div
          ref={(el) => setShellRef(el)}
          style={{ width: "100%", height: "100%", position: "relative", overflow: "hidden" }}
        >
          {/* 1. 边缘触发器 */}
          <Show when={collapsed()}>
            <div
              class="sidebar-trigger"
              onMouseEnter={() => setIsHoveringSidebar(true)}
            />
          </Show>

          {/* 2. 悬停 Overlay */}
          <Show when={collapsed()}>
            <div
              style={{
                position: "absolute",
                top: 0,
                left: 0,
                bottom: 0,
                "z-index": 1000,
                transition: "transform 0.3s ease, box-shadow 0.3s ease",
                transform: isRevealing() ? "translateX(0)" : "translateX(-100%)",
                width: "280px",
                "pointer-events": isRevealing() ? "auto" : "none",
                "box-shadow": isRevealing() ? "4px 0 16px rgba(0,0,0,0.1)" : "none",
                background: "var(--color-bg-surface)"
              }}
              onMouseLeave={() => setIsHoveringSidebar(false)}
            >
              <WorkspaceSidebar
                collapsed={false}
                isOverlay={true}
                fileTreeStyle={fileTreeStyle()}
              />
            </div>
          </Show>

          {/* 3. 主布局：使用 Show 隔离状态 */}
          <Show
            when={!collapsed()}
            fallback={
              <WorkspaceSplitShell
                sidebar={undefined}
                editor={<WorkspaceEditorPane viewMode={viewMode()} />}
                preview={showPreview() ? <WorkspacePreviewPane /> : undefined}
              />
            }
          >
            <WorkspaceSplitShell
              sidebar={
                showSidebar() ? (
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
                if (!showSidebar()) return;
                const next = sizes[0];
                if (typeof next === "number") {
                  setSidebarWidth(next);
                }
              }}
            />
          </Show>
        </div>
      }
      floatingPanel={
        <FloatingEditorPanelContent />
      }
    />
  );
};
