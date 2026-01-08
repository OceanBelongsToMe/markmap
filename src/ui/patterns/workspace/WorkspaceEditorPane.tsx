import { createSignal, Show, createEffect } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { MarkdownEditor } from "../../../features/editor/MarkdownEditor";
import { useActiveDocument } from "../../../state/workspace/useActiveDocument";
import { useWorkspaceTreeState } from "../../../state/workspace/useWorkspaceTree";
import { useWorkspaceActions } from "../../../features/workspace/hooks/useWorkspaceActions";

export const WorkspaceEditorPane = () => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());
  
  const { content, error, activeDocId } = useActiveDocument();
  const { isWorkspaceEmpty } = useWorkspaceTreeState();
  const { importFolder } = useWorkspaceActions();
  
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
          <MarkdownEditor 
            value={editorContent()} 
            onChange={setEditorContent} 
            class="h-full"
          />

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
