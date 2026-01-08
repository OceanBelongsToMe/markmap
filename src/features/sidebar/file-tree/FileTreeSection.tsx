import { useFileTreeState } from "./state/useFileTreeState";
import { useFileTreeActions } from "./state/useFileTreeActions";
import { useWorkspaceFileTree } from "./data/useWorkspaceFileTree";
import { FileTreeView } from "./ui/FileTreeView";
import { useInitialExpand } from "./state/useInitialExpand";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";
import { useActiveDocument } from "../../../state/workspace/useActiveDocument";

export type FileTreeSectionProps = {
  loadingLabel?: string;
  ariaLabel?: string;
  style?: FileTreeStyle;
};

export const FileTreeSection = (props: FileTreeSectionProps) => {
  const { fileNodes, loading } = useWorkspaceFileTree();
  const { expandedIds, setExpandedIds, selectedId, setSelectedId } = useFileTreeState();
  const { openDocument } = useActiveDocument();
  
  const { handleExpandedChange, handleSelectId: baseSelectId } = useFileTreeActions({
    expandedIds,
    setExpandedIds,
    setSelectedId
  });

  const handleSelectId = (id: string) => {
    baseSelectId(id);
    openDocument(id);
  };

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