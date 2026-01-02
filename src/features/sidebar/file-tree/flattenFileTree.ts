import type { FileTreeNode } from "./types";

export type FlatFileTreeNode = {
  node: FileTreeNode;
  depth: number;
  isFolder: boolean;
  isExpanded: boolean;
};

export const flattenFileTree = (
  nodes: FileTreeNode[],
  expandedIds: string[]
): FlatFileTreeNode[] => {
  const result: FlatFileTreeNode[] = [];
  const expanded = new Set(expandedIds);

  const walk = (items: FileTreeNode[], depth: number) => {
    for (const node of items) {
      const isFolder = node.type === "folder";
      const isExpanded = isFolder && expanded.has(node.id);
      result.push({ node, depth, isFolder, isExpanded });
      if (isExpanded && node.children?.length) {
        walk(node.children, depth + 1);
      }
    }
  };

  walk(nodes, 0);
  return result;
};
