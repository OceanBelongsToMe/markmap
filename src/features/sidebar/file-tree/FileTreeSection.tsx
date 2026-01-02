import { useFileTreeData } from "./useFileTreeData";
import { useFileTreeState } from "./useFileTreeState";
import { FileTreeView } from "./FileTreeView";

export const FileTreeSection = () => {
  const { data } = useFileTreeData();
  const { expandedIds, selectedId, setSelectedId, toggleExpanded } = useFileTreeState();

  return (
    <FileTreeView
      nodes={() => data}
      expandedIds={expandedIds}
      selectedId={selectedId}
      onSelect={(id) => setSelectedId(id)}
      onToggle={(id) => toggleExpanded(id)}
    />
  );
};
