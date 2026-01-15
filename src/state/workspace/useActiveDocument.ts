import { createRoot, createSignal } from "solid-js";
import { useDocumentEvents } from "./useDocumentEvents";
import { useWorkspaceTreeState } from "./useWorkspaceTree";
const root = createRoot(() => {
  const [activeDocId, setActiveDocId] = createSignal<string | null>(null);
  const { currentWorkspace } = useWorkspaceTreeState();
  const documentEvents = useDocumentEvents();

  const openDocument = (id: string) => {
    if (activeDocId() === id) return;
    setActiveDocId(id);
    const workspaceId = currentWorkspace()?.workspaceId;
    if (!workspaceId) return;
    documentEvents.publishOpen({
      workspaceId,
      documentId: id,
      openedAt: Date.now()
    });
  };

  const closeDocument = () => {
    setActiveDocId(null);
  };

  return {
    activeDocId,
    openDocument,
    closeDocument,
  };
});

export const useActiveDocument = () => root;
