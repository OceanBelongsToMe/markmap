import { createSignal, createRoot } from "solid-js";

const root = createRoot(() => {
  const [activeDocId, setActiveDocId] = createSignal<string | null>(null);

  const openDocument = (id: string) => {
    setActiveDocId(id);
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