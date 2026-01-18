import type { AstNode, NodeIdAnchor, ResolvedAstNode } from "./types";

function buildLineStarts(doc: string): number[] {
  const starts = [0];
  for (let i = 0; i < doc.length; i += 1) {
    if (doc[i] === "\n") starts.push(i + 1);
  }
  return starts;
}

function lineAt(pos: number, lineStarts: number[]): number {
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
}

function resolveBlockId(anchors: NodeIdAnchor[], line: number): string | undefined {
  for (const anchor of anchors) {
    if (anchor.kind === "block" && anchor.line === line) return anchor.nodeId;
  }
  return undefined;
}

function resolveInlineId(anchors: NodeIdAnchor[], from: number, to: number): string | undefined {
  let best: { nodeId: string; overlap: number } | null = null;
  for (const anchor of anchors) {
    if (anchor.kind !== "inline") continue;
    const overlap = Math.min(anchor.to, to) - Math.max(anchor.from, from);
    if (overlap <= 0) continue;
    if (!best || overlap > best.overlap) {
      best = { nodeId: anchor.nodeId, overlap };
    }
  }
  return best?.nodeId;
}

export function bindNodeIds(
  doc: string,
  astNodes: AstNode[],
  anchors: NodeIdAnchor[]
): ResolvedAstNode[] {
  const lineStarts = buildLineStarts(doc);

  const bind = (node: AstNode): ResolvedAstNode => {
    const line = lineAt(node.from, lineStarts);
    const nodeId =
      node.kind === "text" || node.kind === "emphasis" || node.kind === "strong" || node.kind === "strikethrough" ||
      node.kind === "superscript" || node.kind === "subscript" || node.kind === "inline_code" ||
      node.kind === "link" || node.kind === "image" || node.kind === "html_inline"
        ? resolveInlineId(anchors, node.from, node.to)
        : resolveBlockId(anchors, line);

    return {
      ...node,
      nodeId,
      children: node.children.map(bind),
    };
  };

  return astNodes.map(bind);
}
