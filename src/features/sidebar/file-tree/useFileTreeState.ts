import { createSignal } from "solid-js";

export const useFileTreeState = () => {
  const [expandedIds, setExpandedIds] = createSignal<string[]>([]);
  const [selectedId, setSelectedId] = createSignal<string | null>(null);

  const toggleExpanded = (id: string) => {
    setExpandedIds((current) =>
      current.includes(id) ? current.filter((item) => item !== id) : [...current, id]
    );
  };

  return {
    expandedIds,
    setExpandedIds,
    toggleExpanded,
    selectedId,
    setSelectedId
  };
};
