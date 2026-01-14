import type { INode } from 'markmap-common';
import { walkTree } from 'markmap-common';

export type VisibleNodes = {
  nodes: INode[];
  parentMap: Record<number, number>;
};

export function collectVisibleNodes(rootNode: INode): VisibleNodes {
  const parentMap: Record<number, number> = {};
  const nodes: INode[] = [];
  walkTree(rootNode, (item, next, parent) => {
    if (!item.payload?.fold) next();
    if (parent) parentMap[item.state.id] = parent.state.id;
    nodes.push(item);
  });
  return { nodes, parentMap };
}

export function buildOriginMap(originData: INode | undefined, rootNode: INode) {
  const originMap: Record<number, number> = {};
  if (originData) {
    walkTree(originData, (item, next) => {
      originMap[item.state.id] = originData.state.id;
      next();
    });
  }
  originMap[rootNode.state.id] = rootNode.state.id;
  return originMap;
}

export function buildLinks(nodes: INode[]) {
  return nodes.flatMap((node) =>
    node.payload?.fold
      ? []
      : node.children.map((child) => ({ source: node, target: child })),
  );
}

export function buildHighlightRect(
  highlight: INode,
  _rootNode: INode,
  transform: { k: number },
): { x: number; y: number; width: number; height: number } {
  const padding = 4 / transform.k;
  const rect = { ...highlight.state.rect };
  rect.x -= padding;
  rect.y -= padding;
  rect.width += 2 * padding;
  rect.height += 2 * padding;
  return rect;
}

export function computeIndicatorFilled(node: INode) {
  return Boolean((node.payload as any)?.show_children_indicator);
}
