import { loadCurrentWorkspace, refreshWorkspaceTree } from "../../../state/workspace/useWorkspaceTree";

export const useWorkspaceTreeActions = () => {
  return {
    loadCurrentWorkspace,
    refreshWorkspaceTree
  };
};
