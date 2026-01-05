import { createSignal } from "solid-js";

export const useFileTreeState = () => {
  const [expandedIds, setExpandedIds] = createSignal<string[]>([]);
  const [selectedId, setSelectedId] = createSignal<string | null>(null);

  return {
    expandedIds,
    setExpandedIds,
    selectedId,
    setSelectedId
  };
};
