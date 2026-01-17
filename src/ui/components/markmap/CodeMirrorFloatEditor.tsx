import { Component } from "solid-js";
import { Portal } from "solid-js/web";
import { useCodeMirror } from "../../../lib/codemirror/useCodeMirror";
import { markdown } from "@codemirror/lang-markdown";
import { keymap, EditorView } from "@codemirror/view";
import type { INode } from "markmap-common";

// Duplicate definition until markmap-view types are fully propagated
export interface IEditorArgs {
  node: INode;
  rect: DOMRect;
  initialContent: string;
  save: (newContent: string) => void;
  cancel: () => void;
}

interface Props {
  args: IEditorArgs;
  onClose: () => void;
}

export const CodeMirrorFloatEditor: Component<Props> = (props) => {
  let containerRef: HTMLDivElement | undefined;

  const { view } = useCodeMirror(() => ({
    container: containerRef,
    value: props.args.initialContent,
    autoFocus: true,
    extensions: [
      markdown(),
      EditorView.lineWrapping,
      EditorView.theme({
        "&": {
          backgroundColor: "white",
          color: "black",
          border: "1px solid #ccc",
          borderRadius: "4px",
          boxShadow: "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)",
        },
        ".cm-content": {
          padding: "4px 8px",
          fontFamily: "inherit", // Inherit font from system or app
        },
        "&.cm-focused": {
          outline: "none",
          borderColor: "#3b82f6", // Blue-500
          boxShadow: "0 0 0 2px rgba(59, 130, 246, 0.2)",
        }
      }),
      keymap.of([
        {
          key: "Mod-Enter",
          run: (v) => {
            props.args.save(v.state.doc.toString());
            props.onClose();
            return true;
          },
        },
        {
          key: "Escape",
          run: () => {
            props.args.cancel();
            props.onClose();
            return true;
          },
        },
      ]),
    ],
  }));

  const handleFocusOut = (e: FocusEvent) => {
    // Check if focus moved outside the container
    if (containerRef && !containerRef.contains(e.relatedTarget as Node)) {
      const v = view();
      if (v) {
        props.args.save(v.state.doc.toString());
        props.onClose();
      }
    }
  };

  return (
    <Portal>
      <div
        ref={containerRef}
        onFocusOut={handleFocusOut}
        style={{
          position: "fixed",
          top: `${props.args.rect.top}px`,
          left: `${props.args.rect.left}px`,
          "min-width": `${props.args.rect.width}px`, // At least 200px or original width
          "z-index": 9999,
          "font-size": "14px", // Default size, ideally should match node
          "line-height": "1.5",
        }}
      />
    </Portal>
  );
};
