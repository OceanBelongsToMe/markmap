import * as d3 from 'd3';
import type { INode } from 'markmap-common';
import { childSelector } from '../util';
import type {
  ID3SVGElement,
  IInlineEditorAdapter,
  IInlineEditorSession,
  IMarkmapEditableOptions,
} from '../types';
import { createContentEditableEditor } from './editors/contentEditableEditor';

type EditableState = {
  nodeId: string | number;
  node: INode;
  session: IInlineEditorSession;
};

const activeEditors = new WeakMap<SVGElement, EditableState>();
const defaultEditor = createContentEditableEditor();

function getForeignObjectInner(el: SVGForeignObjectElement): HTMLDivElement | null {
  const outer = el.firstChild as HTMLDivElement | null;
  const inner = outer?.firstChild as HTMLDivElement | null;
  return inner ?? null;
}

function resolveEditor(editable?: IMarkmapEditableOptions): IInlineEditorAdapter {
  if (editable?.editor) return editable.editor;
  if (editable?.renderEditor) {
    return {
      lockPointerEvents: true,
      open: (args) => {
        editable.renderEditor?.(args);
        return {
          close: () => {
            // Legacy renderEditor doesn't expose teardown; rely on args.cancel if needed.
          },
        };
      },
    };
  }
  return defaultEditor;
}

function closeActiveEditor(
  svgNode: SVGElement | null,
  editable: IMarkmapEditableOptions | undefined,
) {
  if (!svgNode) return;
  const current = activeEditors.get(svgNode);
  if (!current) return;
  current.session.close({ cancel: true });
  editable?.onCancel?.(current.nodeId, current.node);
  activeEditors.delete(svgNode);
}

export function measureNodeSizes(
  g: d3.Selection<SVGGElement, INode, HTMLElement, INode>,
  nodeSelector: string,
) {
  g.selectAll<SVGGElement, INode>(childSelector<SVGGElement>(nodeSelector))
    .selectAll<SVGForeignObjectElement, INode>(
      childSelector<SVGForeignObjectElement>('foreignObject'),
    )
    .each(function (d) {
      const el = this.firstChild?.firstChild as HTMLDivElement;
      const newSize: [number, number] = [el.scrollWidth, el.scrollHeight];
      d.state.size = newSize;
    });
}

export function renderHighlight(
  g: d3.Selection<SVGGElement, INode, HTMLElement, INode>,
  highlightSelector: string,
  highlightRect?: { x: number; y: number; width: number; height: number },
) {
  const highlightNodes = g
    .selectAll<SVGGElement, INode>(
      childSelector<SVGGElement>(highlightSelector),
    )
    .selectAll<SVGRectElement, INode>(childSelector<SVGRectElement>('rect'))
    .data(highlightRect ? [highlightRect] : [])
    .join('rect')
    .attr('x', (d) => d.x)
    .attr('y', (d) => d.y)
    .attr('width', (d) => d.width)
    .attr('height', (d) => d.height);
  return highlightNodes;
}

export function renderNodes(args: {
  svg: ID3SVGElement;
  g: d3.Selection<SVGGElement, INode, HTMLElement, INode>;
  nodeSelector: string;
  nodes: INode[];
  nodeMap: Record<number, INode>;
  parentMap: Record<number, number>;
  originMap: Record<number, number>;
  sourceRectMap: Record<
    number,
    { x: number; y: number; width: number; height: number }
  >;
  paddingX: number;
  color: (node: INode) => string;
  maxWidth: number;
  nodeContent?: (node: INode) => string;
  handleClick: (e: MouseEvent, d: INode) => void;
  stopPropagation: (e: Event) => void;
  observer: ResizeObserver;
  editable?: IMarkmapEditableOptions;
}) {
  const {
    svg,
    g,
    nodeSelector,
    nodes,
    nodeMap,
    parentMap,
    originMap,
    sourceRectMap,
    paddingX,
    color,
    maxWidth,
    nodeContent,
    handleClick,
    stopPropagation,
    observer,
    editable,
  } = args;

  const setOriginFromParent = (node: INode) => {
    if (originMap[node.state.id]) return;
    const parentId = parentMap[node.state.id];
    if (parentId) originMap[node.state.id] = parentId;
  };

  const mmG = g
    .selectAll<SVGGElement, INode>(childSelector<SVGGElement>(nodeSelector))
    .each((d) => {
      // Save the current rects before updating nodes
      sourceRectMap[d.state.id] = d.state.rect;
    })
    .data(nodes, (d) => d.state.key);
  const mmGEnter = mmG
    .enter()
    .append('g')
    .attr('data-depth', (d) => d.state.depth)
    .attr('data-path', (d) => d.state.path)
    .each((d) => {
      setOriginFromParent(nodeMap[parentMap[d.state.id]] || d);
    });
  const mmGExit = mmG.exit<INode>().each((d) => {
    setOriginFromParent(nodeMap[parentMap[d.state.id]] || d);
  });
  const mmGMerge = mmG
    .merge(mmGEnter)
    .attr('class', (d) =>
      ['markmap-node', d.payload?.fold && 'markmap-fold']
        .filter(Boolean)
        .join(' '),
    );

  mmGMerge
    .selectAll<SVGLineElement, INode>(childSelector<SVGLineElement>('line'))
    .remove();

  // Circle to link to children of the node
  const mmCircle = mmGMerge
    .selectAll<
      SVGCircleElement,
      INode
    >(childSelector<SVGCircleElement>('circle'))
    .data(
      (d) => {
        return (d.payload as any)?.has_children ? [d] : [];
      },
      (d) => d.state.key,
    );
  const mmCircleEnter = mmCircle
    .enter()
    .append('circle')
    .attr('stroke-width', 0)
    .attr('r', 0)
    .on('click', (e, d) => handleClick(e, d))
    .on('mousedown', stopPropagation);
  const mmCircleMerge = mmCircleEnter
    .merge(mmCircle)
    .attr('stroke', (d) => color(d))
    .attr('fill', (d) => {
      const filled = Boolean((d.payload as any)?.show_children_indicator);
      return filled ? color(d) : 'var(--markmap-circle-open-bg)';
    });
  const mmCircleText = mmGMerge
    .selectAll<
      SVGTextElement,
      INode
    >(childSelector<SVGTextElement>('text.markmap-circle-text'))
    .data(
      (d) => {
        return (d.payload as any)?.has_children ? [d] : [];
      },
      (d) => d.state.key,
    );
  const mmCircleTextEnter = mmCircleText
    .enter()
    .append('text')
    .attr('class', 'markmap-circle-text')
    .attr('opacity', 0);
  const mmCircleTextMerge = mmCircleTextEnter
    .merge(mmCircleText)
    .text((d) => {
      const count = (d.payload as any)?.children_count;
      return typeof count === 'number' ? String(count) : '';
    })
    .attr('fill', (d) => {
      const filled = Boolean((d.payload as any)?.show_children_indicator);
      return filled ? 'var(--markmap-circle-text-filled)' : color(d);
    });
  const mmCircleTextExit = mmCircleText.exit<INode>();

  const mmFo = mmGMerge
    .selectAll<
      SVGForeignObjectElement,
      INode
    >(childSelector<SVGForeignObjectElement>('foreignObject'))
    .data(
      (d) => [d],
      (d) => d.state.key,
    );
  const mmFoEnter = mmFo
    .enter()
    .append('foreignObject')
    .attr('class', 'markmap-foreign')
    .attr('x', paddingX)
    .attr('y', 0)
    .style('opacity', 0)
    .on('mousedown', stopPropagation)
    .on('dblclick', function (event, d) {
      if (!editable?.enabled) {
        stopPropagation(event);
        return;
      }
      event.preventDefault();
      event.stopPropagation();
      const svgNode = svg.node() as SVGElement | null;
      closeActiveEditor(svgNode, editable);
      const inner = getForeignObjectInner(this);
      if (!inner) return;
      const rect = inner.getBoundingClientRect();
      const resolvedId = editable?.getNodeId?.(d) ?? d.state.id;
      const transform = svgNode ? d3.zoomTransform(svgNode) : { k: 1 };
      const editor = resolveEditor(editable);

      let restorePointerEvents = () => {};
      if (svgNode && editor.lockPointerEvents) {
        svgNode.style.pointerEvents = 'none';
        restorePointerEvents = () => {
          svgNode.style.pointerEvents = '';
        };
      }

      const session = editor.open({
        node: d,
        rect,
        k: transform.k,
        paddingX,
        initialContent: d.content,
        host: inner,
        foreignObject: this as SVGForeignObjectElement,
        save: (text) => {
          restorePointerEvents();
          activeEditors.delete(svgNode as SVGElement);
          editable?.onCommit?.(resolvedId, text, d);
        },
        cancel: () => {
          restorePointerEvents();
          activeEditors.delete(svgNode as SVGElement);
          editable?.onCancel?.(resolvedId, d);
        },
      });

      if (!svgNode || !session) return;
      activeEditors.set(svgNode, {
        nodeId: resolvedId,
        node: d,
        session: {
          close: (opts) => {
            restorePointerEvents();
            session.close(opts);
          },
        },
      });
    });
  mmFoEnter
    // The outer `<div>` with a width of `maxWidth`
    .append<HTMLDivElement>('xhtml:div')
    // The inner `<div>` with `display: inline-block` to get the proper width
    .append<HTMLDivElement>('xhtml:div')
    .html((d) => nodeContent?.(d) ?? d.content)
    .attr('xmlns', 'http://www.w3.org/1999/xhtml');
  mmFoEnter.each(function () {
    const el = this.firstChild?.firstChild as Element;
    observer.observe(el);
  });
  const mmFoExit = mmGExit.selectAll<SVGForeignObjectElement, INode>(
    childSelector<SVGForeignObjectElement>('foreignObject'),
  );
  mmFoExit.each(function () {
    const el = this.firstChild?.firstChild as Element;
    observer.unobserve(el);
  });
  const mmFoMerge = mmFoEnter.merge(mmFo);

  svg.style('--markmap-max-width', maxWidth ? `${maxWidth}px` : (null as any));

  return {
    mmGEnter,
    mmGExit,
    mmGMerge,
    mmCircleMerge,
    mmCircleTextMerge,
    mmCircleTextExit,
    mmFoExit,
    mmFoMerge,
  };
}

export function renderLinks(args: {
  g: d3.Selection<SVGGElement, INode, HTMLElement, INode>;
  linkSelector: string;
  links: { source: INode; target: INode }[];
  originMap: Record<number, number>;
  sourceRectMap: Record<
    number,
    { x: number; y: number; width: number; height: number }
  >;
  rootRect: { x: number; y: number; width: number; height: number };
  linkShape: (args: {
    source: [number, number];
    target: [number, number];
  }) => string | null | undefined;
}) {
  const {
    g,
    linkSelector,
    links,
    originMap,
    sourceRectMap,
    rootRect,
    linkShape,
  } = args;
  const getOriginSourceRect = (node: INode) => {
    const originId = originMap[node.state.id];
    return sourceRectMap[originId] || rootRect;
  };
  const mmPath = g
    .selectAll<
      SVGPathElement,
      { source: INode; target: INode }
    >(childSelector<SVGPathElement>(linkSelector))
    .data(links, (d) => d.target.state.key);
  const mmPathExit = mmPath.exit<{ source: INode; target: INode }>();
  const mmPathEnter = mmPath
    .enter()
    .insert('path', 'g')
    .attr('class', 'markmap-link')
    .attr('data-depth', (d) => d.target.state.depth)
    .attr('data-path', (d) => d.target.state.path)
    .attr('d', (d) => {
      const originRect = getOriginSourceRect(d.target);
      const pathOrigin: [number, number] = [
        originRect.x + originRect.width,
        originRect.y + originRect.height / 2,
      ];
      return linkShape({ source: pathOrigin, target: pathOrigin }) ?? null;
    })
    .attr('stroke-width', 0);
  const mmPathMerge = mmPathEnter.merge(mmPath);
  return { mmPathExit, mmPathMerge };
}
