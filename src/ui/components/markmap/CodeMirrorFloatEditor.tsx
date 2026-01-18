import { Component, createEffect } from "solid-js";
import { Portal } from "solid-js/web";
import { useCodeMirror } from "../../../lib/codemirror/useCodeMirror";
import { markdown } from "@codemirror/lang-markdown";
import { keymap, EditorView, drawSelection } from "@codemirror/view";
import { history, historyKeymap, defaultKeymap } from "@codemirror/commands";
import { bracketMatching, indentOnInput } from "@codemirror/language";
import type { IEditorArgs } from "markmap-view";
import type { NodeIdAnchor, ResolvedAst } from "../../../features/markmap/edit/types";
import { nodeIdAnchorField, setNodeIdAnchors } from "../../../features/markmap/edit/nodeIdAnchors";
import { buildResolvedAstFromEditor } from "../../../features/markmap/edit/usecase";
import { extractMarkdownAst } from "../../../features/markmap/edit/astExtractor";
import { applyResolvedAstToParsed } from "../../../features/markmap/edit/resolvedAstBinder";
import { buildAnchorsFromResolvedAst } from "../../../features/markmap/edit/anchorBuilder";
import { syntaxTree } from "@codemirror/language";

interface Props {
  args: IEditorArgs;
  onClose: () => void;
  anchors?: NodeIdAnchor[];
  onSaveResolvedAst?: (ast: ResolvedAst, markdown: string) => void;
  resolvedAst?: { root: { kind: string; node_id: string; children: any[] } };
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
      nodeIdAnchorField,
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        {
          key: "Mod-Enter",
          run: (v) => {
            if (props.onSaveResolvedAst) {
              props.onSaveResolvedAst(buildResolvedAstFromEditor(v), v.state.doc.toString());
            } else {
              props.args.save(v.state.doc.toString());
            }
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
        if (props.onSaveResolvedAst) {
          props.onSaveResolvedAst(buildResolvedAstFromEditor(v), v.state.doc.toString());
        } else {
          props.args.save(v.state.doc.toString());
        }
        props.onClose();
      }
    }
  };

  createEffect(() => {
    const v = view();
    if (!v) return;
    if (props.resolvedAst) {
      const doc = v.state.doc.toString();
      const tree = syntaxTree(v.state);
      const ast = extractMarkdownAst(doc, tree);
      const resolved = applyResolvedAstToParsed(ast, props.resolvedAst.root);
      const anchors = buildAnchorsFromResolvedAst(doc, resolved);
      if (anchors.length) setNodeIdAnchors(v, anchors);
      return;
    }
    if (props.anchors?.length) {
      setNodeIdAnchors(v, props.anchors);
      return;
    }
  });

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
