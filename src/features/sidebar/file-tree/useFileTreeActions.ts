import type { Accessor, Setter } from "solid-js";
import type { FileTreeNode } from "./types";

export type FileTreeActionsArgs = {
  expandedIds: Accessor<string[]>;
  setExpandedIds: Setter<string[]>;
  setSelectedId: Setter<string | null>;
};

export const useFileTreeActions = (args: FileTreeActionsArgs) => {
  const toggleExpanded = (id: string) => {
    args.setExpandedIds((current) =>
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    );
  };

  const handleNodeClick = (node: FileTreeNode) => {
    if (node.type === "folder") {
      toggleExpanded(node.id);
    }
    args.setSelectedId(node.id);
  };

  return {
    handleNodeClick
  };
};
