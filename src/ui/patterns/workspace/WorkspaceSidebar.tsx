import { createMemo, createSignal, onMount } from "solid-js";
import { navModel } from "../../../ia/nav-model";
import { Sidebar } from "../../../layouts/Regions";
import { SidebarShell } from "../../../layouts/SidebarShell";
import { TreeView, type TreeNode } from "../../components/TreeView";
import { SidebarSection } from "../sidebar/SidebarSection";
import { useI18n } from "../../../i18n/context";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceTreeActions } from "../../../features/workspace/hooks/useWorkspaceTreeActions";
import { FileTreeView } from "../../../features/sidebar/file-tree/FileTreeView";
import { useFileTreeState } from "../../../features/sidebar/file-tree/useFileTreeState";
import type { FileTreeNode } from "../../../features/sidebar/file-tree/types";

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
  const { expandedIds, selectedId, setSelectedId, toggleExpanded } = useFileTreeState();
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
  const fileNodes = createMemo<FileTreeNode[]>(() => {
    const tree = fileTree();
    if (!tree) return [];
    return tree.folders.map((folder) => ({
      id: folder.id,
      name: baseName(folder.rootPath),
      type: "folder",
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
            {loading() ? (
              <div>{t("loading")}</div>
            ) : (
              <FileTreeView
                nodes={fileNodes}
                expandedIds={expandedIds}
                selectedId={selectedId}
                onSelect={(id) => setSelectedId(id)}
                onToggle={(id) => toggleExpanded(id)}
              />
            )}
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

const buildPathTree = (folderId: string, files: FileEntry[]): FileTreeNode[] => {
  const root: FileTreeNode[] = [];
  const nodeMap = new Map<string, FileTreeNode>();

  for (const file of files) {
    const parts = file.path.split("/").filter(Boolean);
    let parentKey = folderId;
    let siblings = root;

    parts.forEach((part, index) => {
      const key = `${parentKey}/${part}`;
      let node = nodeMap.get(key);
      if (!node) {
        node = { id: key, name: part, type: "folder" };
        nodeMap.set(key, node);
        siblings.push(node);
      }

      if (index === parts.length - 1) {
        node.id = file.id;
        node.type = "file";
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
