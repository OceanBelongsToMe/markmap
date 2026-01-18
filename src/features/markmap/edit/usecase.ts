import type { EditorView } from "@codemirror/view";
import { syntaxTree } from "@codemirror/language";
import { extractMarkdownAst } from "./astExtractor";
import { bindNodeIds } from "./nodeIdBinder";
import { collectNodeIdAnchors } from "./nodeIdAnchors";
import type { ResolvedAst } from "./types";

export function buildResolvedAstFromEditor(view: EditorView): ResolvedAst {
  const doc = view.state.doc.toString();
  const tree = syntaxTree(view.state);
  const ast = extractMarkdownAst(doc, tree);
  const anchors = collectNodeIdAnchors(view);
  const resolved = bindNodeIds(doc, ast, anchors);

  if (resolved.length === 1) {
    return { root: resolved[0] };
  }

  return {
    root: {
      kind: "unknown",
      from: 0,
      to: doc.length,
      children: resolved,
    },
  };
}
