import { createSignal, createResource, createRoot } from "solid-js";
import { renderDocument } from "../../features/workspace/api/workspaceApi";

const root = createRoot(() => {
  const [activeDocId, setActiveDocId] = createSignal<string | null>(null);

  const fetcher = async (id: string | null) => {
    if (!id) return null;
    const response = await renderDocument(id, "markdown");
    if (response.ok) {
      return response.data.content;
    }
    throw new Error(response.error?.message || "Failed to render document");
  };

  const [documentContent] = createResource(activeDocId, fetcher);

  const openDocument = (id: string) => {
    setActiveDocId(id);
  };

  const closeDocument = () => {
    setActiveDocId(null);
  };

  return {
    activeDocId,
    content: documentContent,
    isLoading: () => documentContent.loading,
    error: () => documentContent.error,
    openDocument,
    closeDocument,
  };
});

export const useActiveDocument = () => root;
