import { createSignal } from "solid-js";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useWorkspaceTreeActions } from "./useWorkspaceTreeActions";

export const useWorkspaceActions = () => {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");
  const { refreshWorkspaceTree, loadCurrentWorkspace } = useWorkspaceTreeActions();

  type WorkspaceAttachFolderResponse = {
    workspace_id: string;
  };

  const greet = async () => {
    try {
      const path = await open({ directory: true });
      if (!path) return;
      const selection = Array.isArray(path) ? path[0] : path;
      if (!selection) return;
      const result = await invoke<WorkspaceAttachFolderResponse>("dispatch", {
        req: { command: "workspace_attach_folder", payload: { root_path: selection } }
      });
      console.log("workspace_attach_folder result", result);
      if (result?.workspace_id) {
        await refreshWorkspaceTree(result.workspace_id);
      } else {
        await loadCurrentWorkspace();
      }
      setGreetMsg(await invoke("greet", { name: selection }));
    } catch (error) {
      console.error("new action error", error);
    }
  };

  return {
    greetMsg,
    name,
    setName,
    greet
  };
};
