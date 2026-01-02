import type { FileTreeNode } from "./types";

export const useFileTreeData = () => {
  const data: FileTreeNode[] = [
    { id: "root", name: "Workspace", type: "folder", children: [] }
  ];

  return {
    data
  };
};
