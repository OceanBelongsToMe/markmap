import { createResource } from "solid-js";
import { renderDocument, type RenderDocumentResponse } from "../../workspace/api/workspaceApi";

type RenderFormat = "markdown" | "html" | "markmap";

type RenderKey = {
  id: string | null;
  format: RenderFormat;
};

const fetcher = async (key: RenderKey): Promise<RenderDocumentResponse["content"] | null> => {
  if (!key.id) return null;
  const response = await renderDocument(key.id, key.format);
  if (response.ok) {
    return response.data.content;
  }
  throw new Error(response.error?.message || `Failed to render document as ${key.format}`);
};

export const useDocumentRender = (id: () => string | null, format: () => RenderFormat = () => "markdown") => {
  const [data] = createResource(
    () => ({ id: id(), format: format() }),
    fetcher
  );

  return {
    data,
    loading: () => data.loading,
    error: () => data.error,
  };
};
