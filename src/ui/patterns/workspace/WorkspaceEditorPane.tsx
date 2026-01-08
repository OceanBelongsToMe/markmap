import { createSignal, Show, createEffect } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { MarkdownEditor } from "../../../features/editor/MarkdownEditor";
import { useActiveDocument } from "../../../state/workspace/useActiveDocument";

export const WorkspaceEditorPane = () => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());
  
  const { content, error, activeDocId } = useActiveDocument();
  
  // Local state for editor value to avoid reactivity loops. 
  // We sync FROM active document TO this signal.
  const [editorContent, setEditorContent] = createSignal("");

  createEffect(() => {
    // When document content loads, update the editor
    const txt = content();
    if (txt !== undefined && txt !== null) {
      // Handle the case where content might be an object (markmap)
      // For MarkdownEditor, we expect a string.
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
      {/* 
        Using flex-1 and min-h-0 is essential for the editor to fill remaining space
        and enable internal scrolling in a flex column container.
      */}
      <div class="flex-1 min-h-0 relative">
        <Show when={activeDocId()} fallback={
          <div class="flex items-center justify-center h-full text-gray-400">
            Select a file to edit
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