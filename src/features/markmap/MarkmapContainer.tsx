import { Component, Show, createResource } from "solid-js";
import { MarkmapCanvas } from "../../ui/components/markmap/MarkmapCanvas";
import { useActiveDocument } from "../../state/workspace/useActiveDocument";
import { defaultOptions } from "markmap-view";
import { nodeContentWithHeadingIcons } from "../../ui/components/markmap/markmapNodeContent";
import { fetchMarkmapChildren, fetchMarkmapRoot } from "../workspace/api/workspaceApi";

export type MarkmapContainerProps = {
  class?: string;
};

const MARKMAP_OPTIONS = {
  ...defaultOptions,
  nodeContent: nodeContentWithHeadingIcons,
};

export const MarkmapContainer: Component<MarkmapContainerProps> = (props) => {
  const { activeDocId } = useActiveDocument();

  const [data] = createResource(
    () => activeDocId(),
    async (docId) => {
      if (!docId) return null;
      const response = await fetchMarkmapRoot(docId);
      if (response.ok) return response.data.content;
      throw new Error(response.error?.message || "Failed to load markmap root");
    }
  );
  const loading = () => data.loading;
  const error = () => data.error;

  const loader = {
    loadChildren: async (nodeId: string) => {
      const docId = activeDocId();
      if (!docId) return [];
      const response = await fetchMarkmapChildren(docId, nodeId);
      if (response.ok) return response.data.content || [];
      throw new Error(response.error?.message || "Failed to load markmap children");
    },
  };

  return (
    <div class={props.class}>
      <Show when={activeDocId()} fallback={
        <div class="flex items-center justify-center h-full text-gray-400">
          Select a file to view mind map
        </div>
      }>
        <Show when={!loading()} fallback={
          <div class="flex items-center justify-center h-full text-gray-400">
            Loading...
          </div>
        }>
          <Show when={data()}>
            <MarkmapCanvas data={data()} options={MARKMAP_OPTIONS} loader={loader} class="h-full" />
          </Show>
          <Show when={error()}>
            <div class="absolute top-0 left-0 right-0 bg-red-100 text-red-800 p-2 z-20">
              Error loading map: {error()?.message}
            </div>
          </Show>
        </Show>
      </Show>
    </div>
  );
};
