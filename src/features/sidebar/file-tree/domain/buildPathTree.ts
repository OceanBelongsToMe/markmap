import type { FileTreeNode } from "./types";
import { parsePath } from "./parsePath";

type FileEntry = {
  id: string;
  path: string;
};

export const buildPathTree = (folderId: string, files: FileEntry[]): FileTreeNode[] => {
  const root: FileTreeNode[] = [];
  const nodeMap = new Map<string, FileTreeNode>();

  for (const file of files) {
    const parts = parsePath(file.path);
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
