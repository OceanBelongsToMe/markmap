import { createMemo, onMount } from "solid-js";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceTreeActions } from "../../../features/workspace/hooks/useWorkspaceTreeActions";
import type { FileTreeNode } from "./types";

export const useWorkspaceFileTree = () => {
  const { fileTree, loading } = useWorkspaceTreeState();
  const { loadCurrentWorkspace } = useWorkspaceTreeActions();

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

  return {
    fileNodes,
    loading
  };
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
