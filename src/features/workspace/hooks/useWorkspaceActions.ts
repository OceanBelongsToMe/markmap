import { open } from "@tauri-apps/plugin-dialog";
import { useWorkspaceTreeActions } from "./useWorkspaceTreeActions";
import { attachFolder } from "../api/workspaceApi";

export const useWorkspaceActions = () => {
  const { refreshWorkspaceTree } = useWorkspaceTreeActions();

  const importFolder = async () => {
    try {
      const path = await open({ directory: true });
      if (!path) return;
      const selection = Array.isArray(path) ? path[0] : path;
      if (!selection) return;

      const response = await attachFolder({ root_path: selection });
      
      if (response.ok && response.data.workspace_id) {
        await refreshWorkspaceTree(response.data.workspace_id);
      }
    } catch (error) {
      console.error("import folder error", error);
    }
  };

  return {
    importFolder
  };
};
