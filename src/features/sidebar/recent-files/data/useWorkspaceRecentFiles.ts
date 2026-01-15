import { createEffect } from "solid-js";
import { useDocumentEvents } from "../../../../state/workspace/useDocumentEvents";
import { useRecentFiles } from "../../../../state/workspace/useRecentFiles";
import { useWorkspaceTreeState } from "../../../../state/workspace/useWorkspaceTree";

export const useWorkspaceRecentFiles = () => {
  const { currentWorkspace } = useWorkspaceTreeState();
  const { lastOpened } = useDocumentEvents();
  const recent = useRecentFiles();

  createEffect(() => {
    const workspaceId = currentWorkspace()?.workspaceId ?? null;
    recent.load(workspaceId);
  });

  createEffect(() => {
    const event = lastOpened();
    const workspaceId = currentWorkspace()?.workspaceId;
    if (!event || !workspaceId || event.workspaceId !== workspaceId) return;
    recent.recordOpen(event.workspaceId, event.documentId, event.openedAt);
  });

  return recent;
};
