import { createMemo, createSignal } from "solid-js";
import { navModel } from "../../../ia/nav-model";
import { Sidebar } from "../../../layouts/Regions";
import { SidebarShell } from "../../../layouts/SidebarShell";
import { TreeView, type TreeNode } from "../../components/TreeView";
import { SidebarSection } from "../sidebar/SidebarSection";
import { useI18n } from "../../../i18n/context";
import { FileTreeSection, type FileTreeStyle } from "../../../features/sidebar/file-tree";
import { RecentFilesSection } from "../../../features/sidebar/recent-files/ui/RecentFilesSection";

export type WorkspaceSidebarProps = {
  collapsed?: boolean;
  onToggle?: () => void;
  fileTreeStyle?: FileTreeStyle;
  isOverlay?: boolean;
  isHidden?: boolean;
  onHover?: (isHovering: boolean) => void;
};

export const WorkspaceSidebar = (props: WorkspaceSidebarProps) => {
  const { t } = useI18n();
  const [navCollapsed, setNavCollapsed] = createSignal(false);
  const [filesCollapsed, setFilesCollapsed] = createSignal(false);
  const [navExpandedIds, setNavExpandedIds] = createSignal(
    navModel.map((item) => item.id)
  );
  
  const nodes = createMemo<TreeNode[]>(() =>
    navModel.map((item) => ({
      id: item.id,
      label: t(item.labelKey),
      children: item.children?.map((child) => ({
        id: child.id,
        label: t(child.labelKey)
      }))
    }))
  );

  return (
    <Sidebar>
      <SidebarShell
        collapsed={props.collapsed}
        overlay={props.isOverlay}
        hidden={props.isHidden}
        onMouseEnter={() => props.onHover?.(true)}
        onMouseLeave={() => props.onHover?.(false)}
      >
        <nav>
          <SidebarSection
            title={t("navigation")}
            collapsed={navCollapsed()}
            onToggle={() => setNavCollapsed(!navCollapsed())}
          >
            <TreeView
              nodes={nodes()}
              ariaLabel={t("navigation")}
              expandedIds={navExpandedIds()}
              onExpandedChange={setNavExpandedIds}
            />
          </SidebarSection>
          <RecentFilesSection
            ariaLabel={t("recent")}
            style={props.fileTreeStyle}
            class="tight-section"
          />
          <SidebarSection
            title={t("files")}
            collapsed={filesCollapsed()}
            onToggle={() => setFilesCollapsed(!filesCollapsed())}
            class="is-grow tight-section files-tight-section"
          >
            <FileTreeSection
              loadingLabel={t("loading")}
              ariaLabel={t("files")}
              style={props.fileTreeStyle}
            />
          </SidebarSection>
        </nav>
      </SidebarShell>
    </Sidebar>
  );
};