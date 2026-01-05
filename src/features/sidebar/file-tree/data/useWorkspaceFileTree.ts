import { createMemo } from "solid-js";
import type { FileTreeNode } from "../domain/types";
import { mapWorkspaceTreeToFileNodes } from "./mapWorkspaceTreeToFileNodes";
import { useWorkspaceFileTreeData } from "./useWorkspaceFileTreeData";

export const useWorkspaceFileTree = () => {
  const { fileTree, loading } = useWorkspaceFileTreeData();

  const fileNodes = createMemo<FileTreeNode[]>(() => {
    return mapWorkspaceTreeToFileNodes(fileTree());
  });

  return {
    fileNodes,
    loading
  };
};
