import { Component, Show, createResource, createSignal } from "solid-js";
import { MarkmapCanvas } from "../../ui/components/markmap/MarkmapCanvas";
import { useActiveDocument } from "../../state/workspace/useActiveDocument";
import { defaultOptions } from "markmap-view";
import { nodeContentWithHeadingIcons } from "../../ui/components/markmap/markmapNodeContent";
import { fetchMarkmapChildren, fetchMarkmapRoot } from "../workspace/api/workspaceApi";
import { CodeMirrorFloatEditor, IEditorArgs } from "../../ui/components/markmap/CodeMirrorFloatEditor";

export type MarkmapContainerProps = {
  class?: string;
};

const BASE_MARKMAP_OPTIONS = {
  ...defaultOptions,
  nodeContent: nodeContentWithHeadingIcons,
  editable: {
    enabled: true,
    getNodeId: (node: any) => node?.payload?.node_id ?? node?.payload?.id,
  },
};

export const MarkmapContainer: Component<MarkmapContainerProps> = (props) => {
  const { activeDocId } = useActiveDocument();
  const [editorArgs, setEditorArgs] = createSignal<IEditorArgs | null>(null);

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

  const markmapOptions = {
    ...BASE_MARKMAP_OPTIONS,
    editable: {
      ...BASE_MARKMAP_OPTIONS.editable,
      onCommit: (nodeId: string | number, text: string) => {
        console.log("[MarkmapContainer] onCommit:", nodeId, text);
        // TODO: Call backend API to update node content
        // await updateNode(String(nodeId), text);
        // refetch();
      },
      renderEditor: (args: any) => {
        setEditorArgs(args);
      },
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
            <MarkmapCanvas
              data={data()}
              options={markmapOptions}
              loader={loader}
              class="h-full"
            />
            <Show when={editorArgs()}>
              {(args) => (
                <CodeMirrorFloatEditor
                  args={args()}
                  onClose={() => setEditorArgs(null)}
                />
              )}
            </Show>
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
