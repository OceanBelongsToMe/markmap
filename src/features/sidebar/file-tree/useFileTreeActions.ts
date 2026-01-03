import type { Accessor, Setter } from "solid-js";
import type { FileTreeNode } from "./types";
import { useTogglePulse } from "./useTogglePulse";

export type FileTreeActionsArgs = {
  expandedIds: Accessor<string[]>;
  setExpandedIds: Setter<string[]>;
  setSelectedId: Setter<string | null>;
};

export const useFileTreeActions = (args: FileTreeActionsArgs) => {
  const { lastToggledId, markToggled } = useTogglePulse();

  const toggleExpanded = (id: string) => {
    args.setExpandedIds((current) =>
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    );
    markToggled(id);
  };

  const handleNodeClick = (node: FileTreeNode) => {
    if (node.type === "folder") {
      toggleExpanded(node.id);
    }
    args.setSelectedId(node.id);
  };

  const handleSelectId = (id: string) => {
    args.setSelectedId(id);
  };

  const handleExpandedChange = (next: string[]) => {
    const current = args.expandedIds();
    const added = next.find((id) => !current.includes(id));
    const removed = current.find((id) => !next.includes(id));
    markToggled(added ?? removed ?? null);
    args.setExpandedIds(next);
  };

  return {
    handleNodeClick,
    handleSelectId,
    handleExpandedChange,
    lastToggledId
  };
};
