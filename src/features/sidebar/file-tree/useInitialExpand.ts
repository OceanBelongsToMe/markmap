import { createEffect, createSignal } from "solid-js";
import type { Accessor, Setter } from "solid-js";
import type { FileTreeNode } from "./types";

type UseInitialExpandArgs = {
  nodes: Accessor<FileTreeNode[]>;
  expandedIds: Accessor<string[]>;
  setExpandedIds: Setter<string[]>;
};

export const useInitialExpand = (args: UseInitialExpandArgs) => {
  const [autoExpandedKey, setAutoExpandedKey] = createSignal<string | null>(null);

  createEffect(() => {
    if (args.expandedIds().length > 0) return;
    const roots = args.nodes();
    if (roots.length === 0) return;
    const rootKey = roots.map((node) => node.id).join("|");
    if (autoExpandedKey() === rootKey) return;
    args.setExpandedIds(roots.map((node) => node.id));
    setAutoExpandedKey(rootKey);
  });
};
