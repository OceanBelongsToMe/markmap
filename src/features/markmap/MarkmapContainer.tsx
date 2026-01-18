import { Component, Show, createResource, createSignal } from "solid-js";
import { MarkmapCanvas } from "../../ui/components/markmap/MarkmapCanvas";
import { useActiveDocument } from "../../state/workspace/useActiveDocument";
import type { IEditorArgs, IInlineEditorAdapter } from "markmap-view";
import { createContentEditableEditor, defaultOptions } from "markmap-view";
import { nodeContentWithHeadingIcons } from "../../ui/components/markmap/markmapNodeContent";
import {
  fetchMarkmapChildren,
  fetchMarkmapEditMarkdown,
  fetchMarkmapNode,
  fetchMarkmapRoot,
  applyMarkmapResolvedAst,
  saveMarkmapEditMarkdown,
  type MarkmapEditMode,
  type MarkmapResolvedAst,
  type MarkmapResolvedAstNode,
  fetchMarkmapEditAnchors,
  type MarkmapNodeIdAnchor,
  fetchMarkmapResolvedAst,
  type MarkmapResolvedAstTree,
} from "../workspace/api/workspaceApi";
import type { MarkmapRenderPort } from "../../ui/components/markmap/markmapRenderer";
import { CodeMirrorFloatEditor } from "../../ui/components/markmap/CodeMirrorFloatEditor";
import type { ResolvedAstNode } from "./edit/types";

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
  const [subtreeEditor, setSubtreeEditor] = createSignal<{
    args: IEditorArgs;
    docId: string;
    nodeId: string;
    anchors: MarkmapNodeIdAnchor[];
    resolvedAst?: MarkmapResolvedAstTree;
  } | null>(null);
  const resolveNodeId = (node: any) =>
    BASE_MARKMAP_OPTIONS.editable.getNodeId(node) ?? node?.state?.id;
  const resolveEditMode = (args: IEditorArgs): MarkmapEditMode =>
    args.triggerEvent?.altKey ? "subtree" : "node";
  let renderPort: MarkmapRenderPort | undefined;
  const toDtoNode = (node: ResolvedAstNode): MarkmapResolvedAstNode => ({
    kind: node.kind,
    node_id: node.nodeId ?? null,
    text: node.text ?? null,
    children: node.children.map(toDtoNode),
  });
  const toDtoAst = (root: ResolvedAstNode): MarkmapResolvedAst => ({
    root: toDtoNode(root),
  });

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
      if (mode === "subtree") {
        let closed = false;
        const start = async () => {
          const response = await fetchMarkmapEditMarkdown(docId, String(nodeId), mode);
          if (!response.ok) {
            console.error("[MarkmapContainer] fetch edit markdown failed", response.error);
            return;
          }
          if (closed) return;
          const wrappedArgs: IEditorArgs = {
            ...args,
            initialContent: response.data.content,
            save: async (content) => {
              const saveResponse = await saveMarkmapEditMarkdown(
                docId,
                String(nodeId),
                "subtree",
                content
              );
              if (!saveResponse.ok) {
                console.error("[MarkmapContainer] save edit markdown failed", saveResponse.error);
                return;
              }
              if (!renderPort) {
                refetch();
                return;
              }
              const childrenResponse = await fetchMarkmapChildren(docId, String(nodeId));
              if (childrenResponse.ok) {
                renderPort.replaceChildren({
                  nodeId: String(nodeId),
                  children: childrenResponse.data.content || []
                });
              } else {
                refetch();
              }
            },
          };
          const anchorsResponse = await fetchMarkmapEditAnchors(docId, String(nodeId));
          const anchors = anchorsResponse.ok ? anchorsResponse.data.anchors : [];
          const resolvedAstResponse = await fetchMarkmapResolvedAst(docId, String(nodeId));
          const resolvedAst = resolvedAstResponse.ok ? resolvedAstResponse.data.ast : undefined;
          setSubtreeEditor({
            args: wrappedArgs,
            docId,
            nodeId: String(nodeId),
            anchors,
            resolvedAst,
          });
        };
        void start();
        return {
          close: (opts) => {
            closed = true;
            if (opts?.cancel) {
              args.cancel();
            }
            setSubtreeEditor(null);
          },
        };
      }
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
            if (!renderPort) {
              refetch();
              return;
            }
            if (mode === "node") {
              const nodeResponse = await fetchMarkmapNode(docId, String(nodeId));
              if (nodeResponse.ok && nodeResponse.data.content) {
                renderPort.replaceNode({
                  nodeId: String(nodeId),
                  node: nodeResponse.data.content
                });
              } else {
                refetch();
              }
              return;
            }
            const childrenResponse = await fetchMarkmapChildren(docId, String(nodeId));
            if (childrenResponse.ok) {
              renderPort.replaceChildren({
                nodeId: String(nodeId),
                children: childrenResponse.data.content || []
              });
            } else {
              refetch();
            }
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
              onRenderer={(port) => {
                renderPort = port;
              }}
              class="h-full"
            />
            <Show when={subtreeEditor()}>
              {(ctx) => (
                <CodeMirrorFloatEditor
                  args={ctx().args}
                  onClose={() => setSubtreeEditor(null)}
                  anchors={ctx().anchors.map((anchor) => ({
                    kind: anchor.kind,
                    line: anchor.line ?? undefined,
                    from: anchor.from ?? undefined,
                    to: anchor.to ?? undefined,
                    nodeId: anchor.node_id,
                  }))}
                  resolvedAst={ctx().resolvedAst}
                  onSaveResolvedAst={async (ast, markdown) => {
                    const resolvedAst = {
                      ...ast,
                      root: {
                        ...ast.root,
                        nodeId: ast.root.nodeId ?? ctx().nodeId,
                      },
                    };
                    const response = await applyMarkmapResolvedAst(
                      ctx().docId,
                      ctx().nodeId,
                      markdown,
                      toDtoAst(resolvedAst.root)
                    );
                    if (!response.ok) {
                      console.error("[MarkmapContainer] apply resolved ast failed", response.error);
                      return;
                    }
                    if (!renderPort) {
                      refetch();
                      return;
                    }
                    const childrenResponse = await fetchMarkmapChildren(
                      ctx().docId,
                      ctx().nodeId
                    );
                    if (childrenResponse.ok) {
                      renderPort.replaceChildren({
                        nodeId: ctx().nodeId,
                        children: childrenResponse.data.content || []
                      });
                    } else {
                      refetch();
                    }
                  }}
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
