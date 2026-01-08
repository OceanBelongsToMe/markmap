import { createSignal, Show, createEffect } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { MarkdownEditor } from "../../../features/editor/MarkdownEditor";
import { useActiveDocument } from "../../../state/workspace/useActiveDocument";
import type { FileTreeStyle } from "../../../features/sidebar/file-tree";

export type WorkspaceEditorPaneProps = {
  fileTreeStyle: FileTreeStyle;
  onFileTreeStyleChange: (style: FileTreeStyle) => void;
};

export const WorkspaceEditorPane = (props: WorkspaceEditorPaneProps) => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());
  
  const { content, isLoading, error, activeDocId } = useActiveDocument();
  
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
      <div style={{ padding: "12px", "border-bottom": "1px solid var(--color-border-subtle)" }}>
        <label style={{ display: "inline-flex", "align-items": "center", gap: "8px" }}>
          <span>File tree style</span>
          <select
            value={props.fileTreeStyle}
            onChange={(event) =>
              props.onFileTreeStyleChange(event.currentTarget.value as FileTreeStyle)
            }
          >
            <option value="ark">ark</option>
            <option value="dense">dense</option>
            <option value="classic">classic</option>
          </select>
        </label>
      </div>
      
      <div class="h-full relative">
        <Show when={activeDocId()} fallback={
          <div class="flex items-center justify-center h-full text-gray-400">
            Select a file to edit
          </div>
        }>
          <Show when={!isLoading()} fallback={
             <div class="absolute inset-0 flex items-center justify-center bg-white/50 z-10">
               Loading...
             </div>
          }>
             <MarkdownEditor 
               value={editorContent()} 
               onChange={setEditorContent} 
               class="h-full"
             />
          </Show>
          <Show when={error()}>
            <div class="absolute top-0 left-0 right-0 bg-red-100 text-red-800 p-2">
              Error loading document: {error()?.message}
            </div>
          </Show>
        </Show>
      </div>
    </EditorPane>
  );
};
