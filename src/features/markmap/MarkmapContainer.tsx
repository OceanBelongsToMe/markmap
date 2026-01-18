import { Component, Show, createResource } from "solid-js";
import { MarkmapCanvas } from "../../ui/components/markmap/MarkmapCanvas";
import { useActiveDocument } from "../../state/workspace/useActiveDocument";
import type { IEditorArgs, IInlineEditorAdapter } from "markmap-view";
import { createContentEditableEditor, defaultOptions } from "markmap-view";
import { nodeContentWithHeadingIcons } from "../../ui/components/markmap/markmapNodeContent";
import {
  fetchMarkmapChildren,
  fetchMarkmapEditMarkdown,
  fetchMarkmapRoot,
  saveMarkmapEditMarkdown,
  type MarkmapEditMode,
} from "../workspace/api/workspaceApi";

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
  const inlineEditor = createContentEditableEditor();
  const resolveNodeId = (node: any) =>
    BASE_MARKMAP_OPTIONS.editable.getNodeId(node) ?? node?.state?.id;
  const resolveEditMode = (args: IEditorArgs): MarkmapEditMode =>
    args.triggerEvent?.altKey ? "subtree" : "node";

  const [data, { refetch }] = createResource(
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

  const editorAdapter: IInlineEditorAdapter = {
    open: (args) => {
      const docId = activeDocId();
      const nodeId = resolveNodeId(args.node);
      if (!docId || !nodeId) return;
      const mode = resolveEditMode(args);
      let closed = false;
      let innerSession: ReturnType<typeof inlineEditor.open> | null = null;

      const start = async () => {
        const response = await fetchMarkmapEditMarkdown(docId, String(nodeId), mode);
        if (!response.ok) {
          console.error("[MarkmapContainer] fetch edit markdown failed", response.error);
          return;
        }
        if (closed) return;
        if (args.host) {
          args.host.textContent = response.data.content;
        }
        innerSession = inlineEditor.open({
          ...args,
          multiline: true,
          commitOnBlur: true,
          save: async (content) => {
            const saveResponse = await saveMarkmapEditMarkdown(
              docId,
              String(nodeId),
              mode,
              content
            );
            if (!saveResponse.ok) {
              console.error("[MarkmapContainer] save edit markdown failed", saveResponse.error);
              return;
            }
            refetch();
          },
          cancel: () => args.cancel(),
        }) || null;
      };

      void start();

      return {
        close: (opts) => {
          closed = true;
          if (innerSession) {
            innerSession.close(opts);
          } else if (opts?.cancel) {
            args.cancel();
          }
        },
      };
    },
  };

  const markmapOptions = {
    ...BASE_MARKMAP_OPTIONS,
    editable: {
      ...BASE_MARKMAP_OPTIONS.editable,
      editor: editorAdapter,
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
