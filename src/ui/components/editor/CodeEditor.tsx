import { Component } from "solid-js";
import { Extension } from "@codemirror/state";
import { useCodeMirror } from "../../../lib/codemirror/useCodeMirror";
import "./code-editor.css"; // We'll create this CSS next

interface CodeEditorProps {
  value: string;
  onChange?: (value: string) => void;
  extensions?: Extension[];
  class?: string;
  autoFocus?: boolean;
}

export const CodeEditor: Component<CodeEditorProps> = (props) => {
  let ref: HTMLDivElement | undefined;

  useCodeMirror(() => ({
    container: ref,
    value: props.value,
    onChange: props.onChange,
    extensions: props.extensions,
    autoFocus: props.autoFocus,
  }));

  return (
    <div
      ref={ref}
      class={`code-editor-root ${props.class || ""}`}
      // Ensure the editor fills the container
      style={{ height: "100%", width: "100%", overflow: "hidden" }}
    />
  );
};
