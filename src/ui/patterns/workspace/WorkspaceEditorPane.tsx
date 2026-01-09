import { createSignal, Show, createEffect } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { MarkdownEditor } from "../../../features/editor/MarkdownEditor";
import { MarkmapContainer } from "../../../features/markmap/MarkmapContainer";
import { useActiveDocument } from "../../../state/workspace/useActiveDocument";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceActions } from "../../../features/workspace/hooks/useWorkspaceActions";
import { useDocumentRender } from "../../../features/document/hooks/useDocumentRender";

type ViewMode = "code" | "markmap";

export const WorkspaceEditorPane = () => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());
  const [viewMode, setViewMode] = createSignal<ViewMode>("code");
  
  const { activeDocId } = useActiveDocument();
  const { isWorkspaceEmpty } = useWorkspaceTreeState();
  const { importFolder } = useWorkspaceActions();
  
  // Fetch markdown only if viewMode is code (or always? Maybe always for fast switch)
  // For now, fetch always to be safe, or conditional.
  // CodeMirror is stateful, so we want to keep it mounted if possible, using CSS to hide?
  // Or just unmount.
  
  const { data: content, error } = useDocumentRender(activeDocId, () => "markdown");
  
  const [editorContent, setEditorContent] = createSignal("");

  createEffect(() => {
    const txt = content();
    if (txt !== undefined && txt !== null) {
      if (typeof txt === 'string') {
         setEditorContent(txt);
      } else {
         setEditorContent(JSON.stringify(txt, null, 2));
      }
    } else if (!activeDocId()) {
      setEditorContent("");
    }
  });

  return (
    <EditorPane ref={(el) => setEditorRef(el)} class={`content-${contentVariant()}`}>
      <div class="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-white">
        <div class="flex gap-2">
          <button 
            class={`px-3 py-1 text-sm rounded ${viewMode() === "code" ? "bg-gray-200 font-medium" : "hover:bg-gray-100"}`}
            onClick={() => setViewMode("code")}
          >
            Code
          </button>
          <button 
            class={`px-3 py-1 text-sm rounded ${viewMode() === "markmap" ? "bg-gray-200 font-medium" : "hover:bg-gray-100"}`}
            onClick={() => setViewMode("markmap")}
          >
            Mind Map
          </button>
        </div>
      </div>

      <div class="flex-1 min-h-0 relative">
        <Show when={activeDocId()} fallback={
          <div class="flex flex-col items-center justify-center h-full text-gray-400 gap-4">
            <Show when={isWorkspaceEmpty()} fallback={
              <span>Select a file to edit</span>
            }>
              <div class="flex flex-col items-center gap-2">
                <span>Your workspace is empty</span>
                <button 
                  onClick={importFolder}
                  class="text-blue-500 hover:text-blue-600 underline cursor-pointer bg-transparent border-none shadow-none p-0 font-normal"
                >
                  Import a folder to get started
                </button>
              </div>
            </Show>
          </div>
        }>
          <div class="h-full" style={{ display: viewMode() === "code" ? "block" : "none" }}>
            <MarkdownEditor 
              value={editorContent()} 
              onChange={setEditorContent} 
              class="h-full"
            />
          </div>
          
          <Show when={viewMode() === "markmap"}>
             <div class="h-full">
                <MarkmapContainer class="h-full" />
             </div>
          </Show>

          <Show when={error()}>
            <div class="absolute top-0 left-0 right-0 bg-red-100 text-red-800 p-2 z-20">
              Error loading document: {error()?.message}
            </div>
          </Show>
        </Show>
      </div>
    </EditorPane>
  );
};
