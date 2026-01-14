import type * as d3 from 'd3';
import type { INode } from 'markmap-common';
import { childSelector } from '../util';

export function applyTransitions<
  T extends d3.BaseType,
  U,
  P extends d3.BaseType,
  Q,
>(sel: d3.Selection<T, U, P, Q>, duration: number): d3.Transition<T, U, P, Q> {
  return sel.transition().duration(duration);
}

export function animateEnterExit(args: {
  duration: number;
  highlightNodes: d3.Selection<SVGRectElement, any, any, any>;
  highlightRect?: { x: number; y: number; width: number; height: number };
  mmGEnter: d3.Selection<SVGGElement, INode, any, any>;
  mmGExit: d3.Selection<SVGGElement, INode, any, any>;
  mmGMerge: d3.Selection<SVGGElement, INode, any, any>;
  mmCircleMerge: d3.Selection<SVGCircleElement, INode, any, any>;
  mmFoExit: d3.Selection<SVGForeignObjectElement, INode, any, any>;
  mmFoMerge: d3.Selection<SVGForeignObjectElement, INode, any, any>;
  mmPathExit: d3.Selection<
    SVGPathElement,
    { source: INode; target: INode },
    any,
    any
  >;
  mmPathMerge: d3.Selection<
    SVGPathElement,
    { source: INode; target: INode },
    any,
    any
  >;
  nodeMap: Record<number, INode>;
  originMap: Record<number, number>;
  sourceRectMap: Record<
    number,
    { x: number; y: number; width: number; height: number }
  >;
  rootSourceRect: { x: number; y: number; width: number; height: number };
  rootTargetRect: { x: number; y: number; width: number; height: number };
  linkShape: (args: {
    source: [number, number];
    target: [number, number];
  }) => string | null | undefined;
  paddingX: number;
  color: (node: INode) => string;
  lineWidth: (node: INode) => number;
}) {
  const {
    duration,
    highlightNodes,
    highlightRect,
    mmGEnter,
    mmGExit,
    mmGMerge,
    mmCircleMerge,
    mmFoExit,
    mmFoMerge,
    mmPathExit,
    mmPathMerge,
    nodeMap,
    originMap,
    sourceRectMap,
    rootSourceRect,
    rootTargetRect,
    linkShape,
    paddingX,
    color,
    lineWidth,
  } = args;

  const transition = <T extends d3.BaseType, U, P extends d3.BaseType, Q>(
    sel: d3.Selection<T, U, P, Q>,
  ) => applyTransitions(sel, duration);

  const updatedHighlights = highlightNodes
    .data(highlightRect ? [highlightRect] : [])
    .join('rect');
  transition(updatedHighlights)
    .attr('x', (d) => d.x)
    .attr('y', (d) => d.y)
    .attr('width', (d) => d.width)
    .attr('height', (d) => d.height);

  const getOriginSourceRect = (node: INode) => {
    const originId = originMap[node.state.id];
    return sourceRectMap[originId] || rootSourceRect;
  };
  const getOriginTargetRect = (node: INode) => {
    const originId = originMap[node.state.id];
    if (originId && nodeMap[originId]) return nodeMap[originId].state.rect;
    return rootTargetRect;
  };

  mmGEnter.attr('transform', (d) => {
    const originRect = getOriginSourceRect(d);
    return `translate(${originRect.x + originRect.width - d.state.rect.width},${
      originRect.y + originRect.height - d.state.rect.height
    })`;
  });
  transition(mmGExit)
    .attr('transform', (d) => {
      const targetRect = getOriginTargetRect(d);
      const targetX = targetRect.x + targetRect.width - d.state.rect.width;
      const targetY = targetRect.y + targetRect.height - d.state.rect.height;
      return `translate(${targetX},${targetY})`;
    })
    .remove();

  transition(mmGMerge).attr(
    'transform',
    (d) => `translate(${d.state.rect.x},${d.state.rect.y})`,
  );

  const mmCircleExit = mmGExit.selectAll<SVGCircleElement, INode>(
    childSelector<SVGCircleElement>('circle'),
  );
  transition(mmCircleExit).attr('r', 0).attr('stroke-width', 0);
  mmCircleMerge
    .attr('cx', (d) => d.state.rect.width)
    .attr('cy', (d) => d.state.rect.height / 2);
  transition(mmCircleMerge).attr('r', 6).attr('stroke-width', '1.5');

  transition(mmFoExit).style('opacity', 0);
  mmFoMerge
    .attr('width', (d) => Math.max(0, d.state.rect.width - paddingX * 2))
    .attr('height', (d) => d.state.rect.height);
  transition(mmFoMerge).style('opacity', 1);

  transition(mmPathExit)
    .attr('d', (d) => {
      const targetRect = getOriginTargetRect(d.target);
      const pathTarget: [number, number] = [
        targetRect.x + targetRect.width,
        targetRect.y + targetRect.height / 2,
      ];
      return linkShape({ source: pathTarget, target: pathTarget }) ?? null;
    })
    .attr('stroke-width', 0)
    .remove();

  transition(mmPathMerge)
    .attr('stroke', (d) => color(d.target))
    .attr('stroke-width', (d) => lineWidth(d.target))
    .attr('d', (d) => {
      const origSource = d.source;
      const origTarget = d.target;
      const source: [number, number] = [
        origSource.state.rect.x + origSource.state.rect.width,
        origSource.state.rect.y + origSource.state.rect.height / 2,
      ];
      const target: [number, number] = [
        origTarget.state.rect.x,
        origTarget.state.rect.y + origTarget.state.rect.height / 2,
      ];
      return linkShape({ source, target }) ?? null;
    });
}
