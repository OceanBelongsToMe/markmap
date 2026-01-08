import { Component, createMemo } from "solid-js";
import { CodeEditor } from "../../ui/components/editor/CodeEditor";
import { defaultExtensions, markdownExtensions, themeExtensions } from "../../lib/codemirror/extensions";

interface MarkdownEditorProps {
  // In a real scenario, we might pass docId instead of raw value/onChange
  value: string;
  onChange: (value: string) => void;
  class?: string;
}

export const MarkdownEditor: Component<MarkdownEditorProps> = (props) => {
  // Combine all extensions
  const extensions = createMemo(() => [
    ...defaultExtensions,
    ...markdownExtensions,
    ...themeExtensions,
  ]);

  return (
    <CodeEditor
      value={props.value}
      onChange={props.onChange}
      extensions={extensions()}
      class={props.class}
      autoFocus
    />
  );
};
