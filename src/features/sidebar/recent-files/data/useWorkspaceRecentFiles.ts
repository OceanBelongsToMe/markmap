import { useRecentFiles } from "../../../../state/workspace/useRecentFiles";

export const useWorkspaceRecentFiles = () => {
  const recent = useRecentFiles();

  return recent;
};
