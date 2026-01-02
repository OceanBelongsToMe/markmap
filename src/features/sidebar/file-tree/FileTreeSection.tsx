import { useFileTreeData } from "./useFileTreeData";
import { useFileTreeState } from "./useFileTreeState";
import { useFileTreeActions } from "./useFileTreeActions";
import { FileTreeView } from "./FileTreeView";

export const FileTreeSection = () => {
  const { data } = useFileTreeData();
  const { expandedIds, setExpandedIds, selectedId, setSelectedId } = useFileTreeState();
  const { handleNodeClick } = useFileTreeActions({
    expandedIds,
    setExpandedIds,
    setSelectedId
  });

  return (
    <FileTreeView
      nodes={() => data}
      expandedIds={expandedIds}
      selectedId={selectedId}
      onNodeClick={handleNodeClick}
    />
  );
};
