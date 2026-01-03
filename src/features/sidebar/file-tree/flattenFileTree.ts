import type { FileTreeNode } from "./types";

export type FlatFileTreeNode = {
  node: FileTreeNode;
  depth: number;
  isFolder: boolean;
  isExpanded: boolean;
  indexPath: number[];
};

export const flattenFileTree = (
  nodes: FileTreeNode[],
  expandedIds: string[]
): FlatFileTreeNode[] => {
  const result: FlatFileTreeNode[] = [];
  const expanded = new Set(expandedIds);

  const walk = (items: FileTreeNode[], depth: number, path: number[]) => {
    items.forEach((node, index) => {
      const indexPath = [...path, index];
      const isFolder = node.type === "folder";
      const isExpanded = isFolder && expanded.has(node.id);
      result.push({ node, depth, isFolder, isExpanded, indexPath });
      if (isExpanded && node.children?.length) {
        walk(node.children, depth + 1, indexPath);
      }
    });
  };

  walk(nodes, 0, []);
  return result;
};
