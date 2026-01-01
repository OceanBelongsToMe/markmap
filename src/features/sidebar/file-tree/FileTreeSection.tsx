import { useFileTreeData } from "./useFileTreeData";
import { useFileTreeState } from "./useFileTreeState";
import { FileTreeView } from "./FileTreeView";

export const FileTreeSection = () => {
  const { data } = useFileTreeData();
  const { selectedId, setSelectedId } = useFileTreeState();

  return (
    <FileTreeView
      nodes={data}
      selectedId={selectedId()}
      onSelect={(id) => setSelectedId(id)}
    />
  );
};
