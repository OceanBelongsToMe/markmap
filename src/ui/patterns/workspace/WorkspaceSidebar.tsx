import { createMemo, createSignal, onMount } from "solid-js";
import { navModel } from "../../../ia/nav-model";
import { Sidebar } from "../../../layouts/Regions";
import { SidebarShell } from "../../../layouts/SidebarShell";
import { TreeView, type TreeNode } from "../../components/TreeView";
import { SidebarSection } from "../sidebar/SidebarSection";
import { useI18n } from "../../../i18n/context";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceTreeActions } from "../../../features/workspace/hooks/useWorkspaceTreeActions";

export type WorkspaceSidebarProps = {
  collapsed?: boolean;
  onToggle?: () => void;
};

export const WorkspaceSidebar = (props: WorkspaceSidebarProps) => {
  const { t } = useI18n();
  const [navCollapsed, setNavCollapsed] = createSignal(false);
  const [filesCollapsed, setFilesCollapsed] = createSignal(false);
  const { fileTree, loading } = useWorkspaceTreeState();
  const { loadCurrentWorkspace } = useWorkspaceTreeActions();
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
  const fileNodes = createMemo<TreeNode[]>(() => {
    const tree = fileTree();
    if (!tree) return [];
    return tree.folders.map((folder) => ({
      id: folder.id,
      label: baseName(folder.rootPath),
      children: buildPathTree(folder.id, folder.documents.map((doc) => ({
        id: doc.id,
        path: doc.path
      })))
    }));
  });

  onMount(() => {
    loadCurrentWorkspace();
  });

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
            {loading() ? <div>{t("loading")}</div> : <TreeView nodes={fileNodes()} />}
          </SidebarSection>
        </nav>
      </SidebarShell>
    </Sidebar>
  );
};

const baseName = (value: string) => {
  const trimmed = value.replace(/\/+$/, "");
  const parts = trimmed.split(/[\\/]/).filter(Boolean);
  return parts[parts.length - 1] ?? value;
};

type FileEntry = {
  id: string;
  path: string;
};

const buildPathTree = (folderId: string, files: FileEntry[]): TreeNode[] => {
  const root: TreeNode[] = [];
  const nodeMap = new Map<string, TreeNode>();

  for (const file of files) {
    const parts = file.path.split("/").filter(Boolean);
    let parentKey = folderId;
    let siblings = root;

    parts.forEach((part, index) => {
      const key = `${parentKey}/${part}`;
      let node = nodeMap.get(key);
      if (!node) {
        node = { id: key, label: part };
        nodeMap.set(key, node);
        siblings.push(node);
      }

      if (index === parts.length - 1) {
        node.id = file.id;
        return;
      }

      if (!node.children) {
        node.children = [];
      }
      parentKey = key;
      siblings = node.children;
    });
  }

  return root;
};
