import { StateEffect, StateField, RangeSetBuilder, Text } from "@codemirror/state";
import { Decoration, DecorationSet, EditorView } from "@codemirror/view";
import type { NodeIdAnchor } from "./types";

const NODE_ID_ATTR = "data-node-id";

const blockDecoration = (nodeId: string) =>
  Decoration.line({ attributes: { [NODE_ID_ATTR]: nodeId } });

const inlineDecoration = (nodeId: string) =>
  Decoration.mark({ attributes: { [NODE_ID_ATTR]: nodeId } });

export function buildNodeIdDecorations(doc: Text, anchors: NodeIdAnchor[]): DecorationSet {
  const builder = new RangeSetBuilder<Decoration>();
  for (const anchor of anchors) {
    if (anchor.kind === "block") {
      const line = doc.line(anchor.line);
      builder.add(line.from, line.from, blockDecoration(anchor.nodeId));
    } else {
      builder.add(anchor.from, anchor.to, inlineDecoration(anchor.nodeId));
    }
  }
  return builder.finish();
}

export const nodeIdAnchorField = StateField.define<DecorationSet>({
  create() {
    return Decoration.none;
  },
  update(value, tr) {
    for (const effect of tr.effects) {
      if (effect.is(setNodeIdAnchorsEffect)) {
        return effect.value;
      }
    }
    if (!tr.docChanged && !tr.annotation(EditorView.focusChange)) {
      return value;
    }
    return value.map(tr.changes);
  },
  provide: (field) => EditorView.decorations.from(field),
});

const setNodeIdAnchorsEffect = StateEffect.define<DecorationSet>();

export function setNodeIdAnchors(view: EditorView, anchors: NodeIdAnchor[]) {
  const decorations = buildNodeIdDecorations(view.state.doc, anchors);
  view.dispatch({
    effects: setNodeIdAnchorsEffect.of(decorations),
  });
}

export function collectNodeIdAnchors(view: EditorView): NodeIdAnchor[] {
  const anchors: NodeIdAnchor[] = [];
  const decorations = view.state.field(nodeIdAnchorField, false);
  if (!decorations) return anchors;

  decorations.between(0, view.state.doc.length, (from, to, deco) => {
    const nodeId = deco.spec.attributes?.[NODE_ID_ATTR];
    if (!nodeId) return;
    if (deco.spec.block) {
      const line = view.state.doc.lineAt(from).number;
      anchors.push({ kind: "block", line, nodeId });
    } else {
      anchors.push({ kind: "inline", from, to, nodeId });
    }
  });

  return anchors;
}
