import { createRoot, createSignal } from "solid-js";

export type DocumentOpenEvent = {
  workspaceId: string;
  documentId: string;
  openedAt: number;
};

const root = createRoot(() => {
  const [lastOpened, setLastOpened] = createSignal<DocumentOpenEvent | null>(null);

  const publishOpen = (event: DocumentOpenEvent) => {
    setLastOpened(event);
  };

  return {
    lastOpened,
    publishOpen
  };
});

export const useDocumentEvents = () => root;
