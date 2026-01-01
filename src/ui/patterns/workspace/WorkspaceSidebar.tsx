import { createMemo } from "solid-js";
import { navModel } from "../../../ia/nav-model";
import { useI18n } from "../../../i18n/context";
import { Sidebar } from "../../../layouts/Regions";
import { SidebarShell } from "../../../layouts/SidebarShell";
import { TreeView, type TreeNode } from "../../components/TreeView";

export const WorkspaceSidebar = () => {
  const { t } = useI18n();
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
      <SidebarShell>
        <nav>
          <TreeView nodes={nodes()} />
        </nav>
      </SidebarShell>
    </Sidebar>
  );
};
