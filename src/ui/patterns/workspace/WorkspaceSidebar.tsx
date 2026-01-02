import { createMemo, createSignal } from "solid-js";
import { navModel } from "../../../ia/nav-model";
import { Sidebar } from "../../../layouts/Regions";
import { SidebarShell } from "../../../layouts/SidebarShell";
import { TreeView, type TreeNode } from "../../components/TreeView";
import { SidebarSection } from "../sidebar/SidebarSection";
import { useI18n } from "../../../i18n/context";
import { FileTreeSection } from "../../../features/sidebar/file-tree/FileTreeSection";

export type WorkspaceSidebarProps = {
  collapsed?: boolean;
  onToggle?: () => void;
};

export const WorkspaceSidebar = (props: WorkspaceSidebarProps) => {
  const { t } = useI18n();
  const [navCollapsed, setNavCollapsed] = createSignal(false);
  const [filesCollapsed, setFilesCollapsed] = createSignal(false);
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
      <SidebarShell collapsed={props.collapsed}>
        <nav>
          <SidebarSection
            title={t("navigation")}
            collapsed={navCollapsed()}
            onToggle={() => setNavCollapsed(!navCollapsed())}
          >
            <TreeView nodes={nodes()} />
          </SidebarSection>
          <SidebarSection
            title={t("files")}
            collapsed={filesCollapsed()}
            onToggle={() => setFilesCollapsed(!filesCollapsed())}
          >
            <FileTreeSection loadingLabel={t("loading")} />
          </SidebarSection>
        </nav>
      </SidebarShell>
    </Sidebar>
  );
};
