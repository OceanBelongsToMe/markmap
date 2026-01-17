import { Component } from "solid-js";
import { Portal } from "solid-js/web";
import { useCodeMirror } from "../../../lib/codemirror/useCodeMirror";
import { markdown } from "@codemirror/lang-markdown";
import { keymap, EditorView, drawSelection } from "@codemirror/view";
import { history, historyKeymap, defaultKeymap } from "@codemirror/commands";
import { bracketMatching, indentOnInput } from "@codemirror/language";
import type { INode } from "markmap-common";

export interface IEditorArgs {
  node: INode;
  rect: DOMRect;
  k: number;
  paddingX: number;
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
    useBasicSetup: false,
    extensions: [
      markdown(),
      EditorView.lineWrapping,
      history(),
      drawSelection(),
      bracketMatching(),
      indentOnInput(),
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
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
      EditorView.theme({
        "&": {
          backgroundColor: "white",
          color: "black",
          border: "1px solid #3b82f6",
          borderRadius: "4px",
          boxShadow: "0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05)",
          height: `${props.args.rect.height + 4 * props.args.k}px`,
          maxHeight: `${props.args.rect.height + 4 * props.args.k}px`,
          display: "flex",
          flexDirection: "column",
        },
        ".cm-scroller": {
          overflowY: "auto",
          overflowX: "hidden",
        },
        ".cm-content": {
          padding: `0px ${4 * props.args.k}px ${2 * props.args.k}px ${2 * props.args.k}px`,
          fontFamily: "ui-sans-serif, system-ui, sans-serif",
          fontSize: `${14 * props.args.k}px`,
        },
        "&.cm-focused": {
          outline: "none",
        },
      }),
    ],
  }));

  const handleFocusOut = (e: FocusEvent) => {
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
          top: `${props.args.rect.top + props.args.rect.height / 2}px`,
          left: `${props.args.rect.left - props.args.paddingX * props.args.k}px`,
          transform: "translateY(-50%)",
          "min-width": `${props.args.rect.width + props.args.paddingX * props.args.k}px`,
          "z-index": 9999,
        }}
      />
    </Portal>
  );
};
