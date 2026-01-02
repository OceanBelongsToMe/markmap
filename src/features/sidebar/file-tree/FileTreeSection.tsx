import { useFileTreeState } from "./useFileTreeState";
import { useFileTreeActions } from "./useFileTreeActions";
import { FileTreeView } from "./FileTreeView";
import { useWorkspaceFileTree } from "./useWorkspaceFileTree";

export type FileTreeSectionProps = {
  loadingLabel?: string;
};

export const FileTreeSection = (props: FileTreeSectionProps) => {
  const { fileNodes, loading } = useWorkspaceFileTree();
  const { expandedIds, setExpandedIds, selectedId, setSelectedId } = useFileTreeState();
  const { handleNodeClick } = useFileTreeActions({
    expandedIds,
    setExpandedIds,
    setSelectedId
  });

  if (loading()) {
    return <div>{props.loadingLabel ?? "Loading..."}</div>;
  }

  return (
    <FileTreeView
      nodes={fileNodes}
      expandedIds={expandedIds}
      selectedId={selectedId}
      onNodeClick={handleNodeClick}
    />
  );
};
