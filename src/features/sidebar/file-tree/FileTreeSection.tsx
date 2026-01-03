import { createEffect } from "solid-js";
import { useFileTreeState } from "./useFileTreeState";
import { useFileTreeActions } from "./useFileTreeActions";
import { useWorkspaceFileTree } from "./useWorkspaceFileTree";
import { FileTreeView } from "./FileTreeView";

export type FileTreeSectionProps = {
  loadingLabel?: string;
  ariaLabel?: string;
};

export const FileTreeSection = (props: FileTreeSectionProps) => {
  const { fileNodes, loading } = useWorkspaceFileTree();
  const { expandedIds, setExpandedIds, selectedId, setSelectedId } = useFileTreeState();
  const { handleExpandedChange, handleSelectId } = useFileTreeActions({
    expandedIds,
    setExpandedIds,
    setSelectedId
  });

  createEffect(() => {
    if (expandedIds().length > 0) return;
    const roots = fileNodes();
    if (roots.length === 0) return;
    setExpandedIds(roots.map((node) => node.id));
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
    />
  );
};
