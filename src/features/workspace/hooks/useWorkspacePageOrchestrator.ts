import { createEffect, onMount } from "solid-js";
import { useWorkspaceTreeActions } from "./useWorkspaceTreeActions";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useRecentFiles } from "../../../state/workspace/useRecentFiles";
import { useDocumentEvents } from "../../../state/workspace/useDocumentEvents";

export const useWorkspacePageOrchestrator = () => {
  const { loadCurrentWorkspace } = useWorkspaceTreeActions();
  const { currentWorkspace } = useWorkspaceTreeState();
  const recentFiles = useRecentFiles();
  const { lastOpened } = useDocumentEvents();

  onMount(() => {
    loadCurrentWorkspace();
  });

  createEffect(() => {
    const workspaceId = currentWorkspace()?.workspaceId ?? null;
    recentFiles.load(workspaceId);
  });

  createEffect(() => {
    const event = lastOpened();
    const workspaceId = currentWorkspace()?.workspaceId;
    if (!event || !workspaceId || event.workspaceId !== workspaceId) return;
    recentFiles.recordOpen(event.workspaceId, event.documentId, event.openedAt);
  });

  return {};
};
