import type { AstNode, ResolvedAstNode } from "./types";

type ResolvedAstSource = {
  kind: string;
  node_id: string;
  children: ResolvedAstSource[];
};

function kindsMatch(astKind: string, sourceKind: string): boolean {
  return astKind === sourceKind;
}

export function applyResolvedAstToParsed(
  ast: AstNode[],
  source: ResolvedAstSource
): ResolvedAstNode[] {
  const bind = (node: AstNode, src: ResolvedAstSource): ResolvedAstNode => {
    const children: ResolvedAstNode[] = [];
    const count = Math.min(node.children.length, src.children.length);
    for (let i = 0; i < count; i += 1) {
      children.push(bind(node.children[i], src.children[i]));
    }
    return {
      ...node,
      nodeId: kindsMatch(node.kind, src.kind) ? src.node_id : undefined,
      children,
    };
  };

  if (ast.length === 1) {
    return [bind(ast[0], source)];
  }
  const result: ResolvedAstNode[] = [];
  const count = Math.min(ast.length, source.children.length);
  for (let i = 0; i < count; i += 1) {
    result.push(bind(ast[i], source.children[i]));
  }
  return result;
}
