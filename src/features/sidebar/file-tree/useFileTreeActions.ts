import { createSignal, onCleanup } from "solid-js";
import type { Accessor, Setter } from "solid-js";
import type { FileTreeNode } from "./types";

export type FileTreeActionsArgs = {
  expandedIds: Accessor<string[]>;
  setExpandedIds: Setter<string[]>;
  setSelectedId: Setter<string | null>;
};

export const useFileTreeActions = (args: FileTreeActionsArgs) => {
  const [lastToggledId, setLastToggledId] = createSignal<string | null>(null);
  let clearTimer: number | undefined;

  const toggleExpanded = (id: string) => {
    args.setExpandedIds((current) =>
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    );
    setLastToggledId(id);
    if (clearTimer) {
      window.clearTimeout(clearTimer);
    }
    clearTimer = window.setTimeout(() => setLastToggledId(null), 220);
  };

  onCleanup(() => {
    if (clearTimer) {
      window.clearTimeout(clearTimer);
    }
  });

  const handleNodeClick = (node: FileTreeNode) => {
    if (node.type === "folder") {
      toggleExpanded(node.id);
    }
    args.setSelectedId(node.id);
  };

  return {
    handleNodeClick,
    lastToggledId
  };
};
