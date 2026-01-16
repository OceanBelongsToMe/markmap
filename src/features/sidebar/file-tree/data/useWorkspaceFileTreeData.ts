import { useWorkspaceTreeState } from "../../../../state/workspace/useWorkspaceTree";

export const useWorkspaceFileTreeData = () => {
  const { fileTree, loading } = useWorkspaceTreeState();

  return {
    fileTree,
    loading
  };
};
