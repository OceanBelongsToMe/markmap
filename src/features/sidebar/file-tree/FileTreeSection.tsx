import { useFileTreeState } from "./useFileTreeState";
import { useFileTreeActions } from "./useFileTreeActions";
import { useWorkspaceFileTree } from "./useWorkspaceFileTree";
import { FileTreeView } from "./FileTreeView";
import { useInitialExpand } from "./useInitialExpand";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";

export type FileTreeSectionProps = {
  loadingLabel?: string;
  ariaLabel?: string;
  style?: FileTreeStyle;
};

export const FileTreeSection = (props: FileTreeSectionProps) => {
  const { fileNodes, loading } = useWorkspaceFileTree();
  const { expandedIds, setExpandedIds, selectedId, setSelectedId } = useFileTreeState();
  const { handleExpandedChange, handleSelectId } = useFileTreeActions({
    expandedIds,
    setExpandedIds,
    setSelectedId
  });
  useInitialExpand({
    nodes: fileNodes,
    expandedIds,
    setExpandedIds
  });

  if (loading()) {
    return <div>{props.loadingLabel ?? "Loading..."}</div>;
  }

  return (
    <FileTreeView
      nodes={fileNodes}
      ariaLabel={props.ariaLabel}
      selectedId={selectedId}
      expandedIds={expandedIds}
      onSelect={handleSelectId}
      onExpandedChange={handleExpandedChange}
      style={props.style}
    />
  );
};
