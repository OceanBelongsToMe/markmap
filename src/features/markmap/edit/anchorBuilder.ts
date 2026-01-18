import type { ResolvedAstNode, NodeIdAnchor } from "./types";

export function buildAnchorsFromResolvedAst(
  doc: string,
  ast: ResolvedAstNode[]
): NodeIdAnchor[] {
  const anchors: NodeIdAnchor[] = [];
  const lineStarts: number[] = [0];
  for (let i = 0; i < doc.length; i += 1) {
    if (doc[i] === "\n") lineStarts.push(i + 1);
  }

  const lineAt = (pos: number) => {
    let low = 0;
    let high = lineStarts.length - 1;
    while (low <= high) {
      const mid = Math.floor((low + high) / 2);
      const start = lineStarts[mid];
      const next = mid + 1 < lineStarts.length ? lineStarts[mid + 1] : Number.POSITIVE_INFINITY;
      if (pos >= start && pos < next) return mid + 1;
      if (pos < start) high = mid - 1;
      else low = mid + 1;
    }
    return 1;
  };

  const isInline = (kind: string) =>
    [
      "text",
      "emphasis",
      "strong",
      "strikethrough",
      "superscript",
      "subscript",
      "inline_code",
      "link",
      "image",
      "html_inline",
    ].includes(kind);

  const visit = (node: ResolvedAstNode) => {
    if (node.nodeId) {
      if (isInline(node.kind)) {
        anchors.push({ kind: "inline", from: node.from, to: node.to, nodeId: node.nodeId });
      } else {
        anchors.push({ kind: "block", line: lineAt(node.from), nodeId: node.nodeId });
      }
    }
    node.children.forEach(visit);
  };

  ast.forEach(visit);
  return anchors;
}
