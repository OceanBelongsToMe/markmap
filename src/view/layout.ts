import { max, min } from 'd3';
import { flextree } from 'd3-flextree';
import type { INode } from 'markmap-common';
import type { IMarkmapOptions } from '../types';

export type LayoutRect = {
  x: number;
  y: number;
  width: number;
  height: number;
};
export type LayoutBounds = { x1: number; x2: number; y1: number; y2: number };
export type LayoutResult = {
  rects: Array<{ id: number; rect: LayoutRect }>;
  bounds: LayoutBounds;
};

export function computeLayout(
  rootNode: INode,
  options: IMarkmapOptions,
): LayoutResult {
  const { lineWidth, paddingX, spacingHorizontal, spacingVertical } = options;
  const layout = flextree<INode>({})
    .children((d) => {
      if (!d.payload?.fold) return d.children;
    })
    .nodeSize((node) => {
      const [width, height] = node.data.state.size;
      return [height, width + (width ? paddingX * 2 : 0) + spacingHorizontal];
    })
    .spacing((a, b) => {
      return (
        (a.parent === b.parent ? spacingVertical : spacingVertical * 2) +
        lineWidth(a.data)
      );
    });
  const tree = layout.hierarchy(rootNode);
  layout(tree);
  const fnodes = tree.descendants();
  const rects = fnodes.map((fnode) => ({
    id: fnode.data.state.id,
    rect: {
      x: fnode.y,
      y: fnode.x - fnode.xSize / 2,
      width: fnode.ySize - spacingHorizontal,
      height: fnode.xSize,
    },
  }));
  const bounds = {
    x1: min(rects, (item) => item.rect.x) || 0,
    y1: min(rects, (item) => item.rect.y) || 0,
    x2: max(rects, (item) => item.rect.x + item.rect.width) || 0,
    y2: max(rects, (item) => item.rect.y + item.rect.height) || 0,
  };
  return { rects, bounds };
}

export function applyRectsToNodes(
  rects: Array<{ id: number; rect: LayoutRect }>,
  nodes: INode[],
) {
  const rectMap = new Map<number, LayoutRect>();
  rects.forEach((item) => {
    rectMap.set(item.id, item.rect);
  });
  nodes.forEach((node) => {
    const rect = rectMap.get(node.state.id);
    if (rect) node.state.rect = rect;
  });
}
