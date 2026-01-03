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

  const markToggled = (id: string | null) => {
    if (!id) return;
    setLastToggledId(id);
    if (clearTimer) {
      window.clearTimeout(clearTimer);
    }
    clearTimer = window.setTimeout(() => setLastToggledId(null), 220);
  };

  const toggleExpanded = (id: string) => {
    args.setExpandedIds((current) =>
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    );
    markToggled(id);
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
