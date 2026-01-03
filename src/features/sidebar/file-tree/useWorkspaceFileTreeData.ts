import { onMount } from "solid-js";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceTreeActions } from "../../../features/workspace/hooks/useWorkspaceTreeActions";

export const useWorkspaceFileTreeData = () => {
  const { fileTree, loading } = useWorkspaceTreeState();
  const { loadCurrentWorkspace } = useWorkspaceTreeActions();

  onMount(() => {
    loadCurrentWorkspace();
  });

  return {
    fileTree,
    loading
  };
};
