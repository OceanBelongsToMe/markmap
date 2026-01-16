import type * as d3 from 'd3';
import {
  linkHorizontal,
  minIndex,
  select,
  zoom,
  zoomIdentity,
  zoomTransform,
} from 'd3';
import {
  Hook,
  INode,
  IPureNode,
  addClass,
  debounce,
  getId,
  noop,
  walkTree,
} from 'markmap-common';
import { defaultOptions, isMacintosh } from './constants';
import css from './style.css?inline';
import {
  ID3SVGElement,
  IMarkmapOptions,
  IMarkmapState,
  INodeLoader,
  IPadding,
} from './types';
import { childSelector, simpleHash } from './util';
import {
  buildHighlightRect,
  buildLinks,
  buildOriginMap,
  collectVisibleNodes,
} from './view/data';
import { applyRectsToNodes, computeLayout } from './view/layout';
import { animateEnterExit } from './view/animation';
import {
  measureNodeSizes,
  renderHighlight,
  renderLinks,
  renderNodes,
} from './view/render';

export const globalCSS = css;

const SELECTOR_NODE = 'g.markmap-node';
const SELECTOR_LINK = 'path.markmap-link';
const SELECTOR_HIGHLIGHT = 'g.markmap-highlight';

const linkShape = linkHorizontal();

function minBy(numbers: number[], by: (v: number) => number): number {
  const index = minIndex(numbers, by);
  return numbers[index];
}

function stopPropagation(e: Event) {
  e.stopPropagation();
}

/**
 * A global hook to refresh all markmaps when called.
 */
export const refreshHook = new Hook<[]>();

export class Markmap {
  options = { ...defaultOptions };

  state: IMarkmapState;

  svg: ID3SVGElement;

  styleNode: d3.Selection<HTMLStyleElement, INode, HTMLElement, INode>;

  g: d3.Selection<SVGGElement, INode, HTMLElement, INode>;

  zoom: d3.ZoomBehavior<SVGElement, INode>;

  private _observer: ResizeObserver;

  private _nodeLoader?: INodeLoader;

  private _disposeList: (() => void)[] = [];

  constructor(
    svg: string | SVGElement | ID3SVGElement,
    opts?: Partial<IMarkmapOptions>,
  ) {
    this.svg = (svg as ID3SVGElement).datum
      ? (svg as ID3SVGElement)
      : select(svg as string);
    this.styleNode = this.svg.append('style');
    this.zoom = zoom<SVGElement, INode>()
      .filter((event) => {
        if (this.options.scrollForPan) {
          // Pan with wheels, zoom with ctrl+wheels
          if (event.type === 'wheel') return event.ctrlKey && !event.button;
        }
        return (!event.ctrlKey || event.type === 'wheel') && !event.button;
      })
      .on('zoom', this.handleZoom);
    this.setOptions(opts);
    this.state = {
      id: this.options.id || this.svg.attr('id') || getId(),
      rect: { x1: 0, y1: 0, x2: 0, y2: 0 },
    };
    this.g = this.svg.append('g');
    this.g.append('g').attr('class', 'markmap-highlight');
    this._observer = new ResizeObserver(
      debounce(() => {
        this.renderData();
      }, 100),
    );
    this._disposeList.push(
      refreshHook.tap(() => {
        this.setData();
      }),
      () => this._observer.disconnect(),
    );
  }

  getStyleContent(): string {
    const { style } = this.options;
    const { id } = this.state;
    const styleText = typeof style === 'function' ? style(id) : '';
    return [this.options.embedGlobalCSS && css, styleText]
      .filter(Boolean)
      .join('\n');
  }

  updateStyle(): void {
    this.svg.attr(
      'class',
      addClass(this.svg.attr('class'), 'markmap', this.state.id),
    );
    const style = this.getStyleContent();
    this.styleNode.text(style);
  }

  handleZoom = (e: any) => {
    const { transform } = e;
    this.g.attr('transform', transform);
  };

  handlePan = (e: WheelEvent) => {
    e.preventDefault();
    const transform = zoomTransform(this.svg.node()!);
    const newTransform = transform.translate(
      -e.deltaX / transform.k,
      -e.deltaY / transform.k,
    );
    this.svg.call(this.zoom.transform, newTransform);
  };

  async toggleNode(data: INode, recursive = false) {
    const shouldLoad = this._shouldLoadChildren(data);
    if (shouldLoad) await this._loadAndAttachChildren(data);
    const fold = this._computeNextFold(data, { forceOpen: shouldLoad });
    this._applyFold(data, fold, recursive);
    await this.renderData(data);
  }

  handleClick = (e: MouseEvent, d: INode) => {
    let recursive = this.options.toggleRecursively;
    if (isMacintosh ? e.metaKey : e.ctrlKey) recursive = !recursive;
    this.toggleNode(d, recursive);
  };

  setNodeLoader(loader?: INodeLoader) {
    this._nodeLoader = loader;
  }

  private _shouldLoadChildren(node: INode) {
    const hasChildren = Boolean((node.payload as any)?.has_children);
    const childrenLoaded = Boolean((node.payload as any)?.children_loaded);
    return Boolean(hasChildren && !childrenLoaded && this._nodeLoader);
  }

  private async _loadAndAttachChildren(parent: INode) {
    if (!this._nodeLoader) return;
    const nodeId = (parent.payload as any)?.node_id ?? parent.state.key;
    const children = await this._nodeLoader.loadChildren(String(nodeId));
    this.addNode(parent, children);
  }

  private _computeNextFold(node: INode, opts: { forceOpen: boolean }) {
    if (opts.forceOpen) return 0;
    return node.payload?.fold ? 0 : 1;
  }

  private _applyFold(node: INode, fold: number, recursive: boolean) {
    if (recursive) {
      // recursively
      walkTree(node, (item, next) => {
        item.payload = {
          ...item.payload,
          fold,
        };
        this._updateIndicatorFromPayload(item);
        next();
      });
    } else {
      node.payload = {
        ...node.payload,
        fold,
      };
      this._updateIndicatorFromPayload(node);
    }
  }

  private _getMaxStateId(rootNode: INode) {
    let maxId = 0;
    walkTree(rootNode, (item, next) => {
      if (item.state?.id && item.state.id > maxId) maxId = item.state.id;
      next();
    });
    return maxId;
  }

  private _reindexSubtree(
    parent: INode,
    children: INode[],
    nextIdRef: { value: number },
  ) {
    const parentPath = parent.state?.path || `${parent.state?.id || 0}`;
    const parentDepth = parent.state?.depth || 0;
    children.forEach((child) => {
      const id = (nextIdRef.value += 1);
      const path = `${parentPath}.${id}`;
      child.state = {
        ...child.state,
        id,
        depth: parentDepth + 1,
        path,
      };
      child.payload = {
        ...child.payload,
        path,
      };
      if (child.children?.length) {
        this._reindexSubtree(child, child.children, nextIdRef);
      }
    });
  }

  addNode(parent: INode, children: INode[]) {
    const normalizedChildren = children ?? [];
    this._normalizeSubtreeIds(parent, normalizedChildren);
    this._attachChildren(parent, normalizedChildren);
    this._updateChildStats(parent, normalizedChildren);
  }

  private _normalizeSubtreeIds(parent: INode, children: INode[]) {
    const rootNode = this.state.data;
    if (!rootNode || !children.length) return;
    const nextIdRef = { value: this._getMaxStateId(rootNode) };
    this._reindexSubtree(parent, children, nextIdRef);
  }

  private _attachChildren(parent: INode, children: INode[]) {
    parent.children = children;
  }

  private _updateChildStats(parent: INode, children: INode[]) {
    parent.payload = {
      ...parent.payload,
      children_loaded: true,
      has_children: children.length > 0,
      children_count: children.length,
    } as any;
    this._updateIndicatorFromPayload(parent);
  }

  private _updateIndicatorFromPayload(node: INode) {
    const payload = node.payload as any;
    if (payload?.has_children === undefined) return;
    const hasChildren = Boolean(payload.has_children);
    const isFolded = Boolean(payload.fold);
    const needsLazyIndicator = payload.children_loaded === false;
    payload.show_children_indicator =
      hasChildren && (isFolded || needsLazyIndicator);
  }

  private _initializeData(node: IPureNode | INode) {
    let nodeId = 0;
    const { color, initialExpandLevel } = this.options;

    let foldRecursively = 0;
    let depth = 0;
    walkTree(node as INode, (item, next, parent) => {
      depth += 1;
      item.children = item.children?.map((child) => ({ ...child }));
      nodeId += 1;
      item.state = {
        ...item.state,
        depth,
        id: nodeId,
        rect: {
          x: 0,
          y: 0,
          width: 0,
          height: 0,
        },
        size: [0, 0],
      };
      item.state.key =
        [parent?.state?.id, item.state.id].filter(Boolean).join('.') +
        simpleHash(item.content);
      item.state.path = [parent?.state?.path, item.state.id]
        .filter(Boolean)
        .join('.');
      color(item); // preload colors

      const isFoldRecursively = item.payload?.fold === 2;
      if (isFoldRecursively) {
        foldRecursively += 1;
      } else if (
        foldRecursively ||
        (initialExpandLevel >= 0 && item.state.depth >= initialExpandLevel)
      ) {
        item.payload = { ...item.payload, fold: 1 };
      }
      next();
      if (isFoldRecursively) foldRecursively -= 1;
      depth -= 1;
    });

    return node as INode;
  }

  setOptions(opts?: Partial<IMarkmapOptions>): void {
    this.options = {
      ...this.options,
      ...opts,
    };
    if (this.options.zoom) {
      this.svg.call(this.zoom);
    } else {
      this.svg.on('.zoom', null);
    }
    if (this.options.pan) {
      this.svg.on('wheel', this.handlePan);
    } else {
      this.svg.on('wheel', null);
    }
  }

  async setData(data?: IPureNode | null, opts?: Partial<IMarkmapOptions>) {
    if (opts) this.setOptions(opts);
    if (data) this.state.data = this._initializeData(data);
    if (!this.state.data) return;
    this.updateStyle();
    await this.renderData();
  }

  async setHighlight(node?: INode | null) {
    this.state.highlight = node || undefined;
    await this.renderData();
  }

  async renderData(originData?: INode) {
    const { paddingX, autoFit, color, lineWidth } = this.options;
    const rootNode = this.state.data;
    if (!rootNode) return;

    const { nodes, parentMap } = collectVisibleNodes(rootNode);
    const nodeMap: Record<number, INode> = {};
    nodes.forEach((node) => {
      nodeMap[node.state.id] = node;
    });
    const originMap = buildOriginMap(originData, rootNode);
    const sourceRectMap: Record<
      number,
      { x: number; y: number; width: number; height: number }
    > = {
      [rootNode.state.id]: rootNode.state.rect,
    };
    const svgNode = this.svg.node()!;
    const transform = zoomTransform(svgNode);
    const highlight = this.state.highlight;
    const highlightVisible =
      highlight && nodeMap[highlight.state.id] ? highlight : undefined;
    const initialHighlightRect = highlightVisible
      ? buildHighlightRect(highlightVisible, rootNode, transform)
      : undefined;
    const highlightNodes = renderHighlight(
      this.g,
      SELECTOR_HIGHLIGHT,
      initialHighlightRect,
    );
    const {
      mmGEnter,
      mmGExit,
      mmGMerge,
      mmCircleMerge,
      mmCircleTextMerge,
      mmCircleTextExit,
      mmFoExit,
      mmFoMerge,
    } = renderNodes({
      svg: this.svg,
      g: this.g,
      nodeSelector: SELECTOR_NODE,
      nodes,
      nodeMap,
      parentMap,
      originMap,
      sourceRectMap,
      paddingX,
      color,
      maxWidth: this.options.maxWidth,
      nodeContent: this.options.nodeContent,
      handleClick: this.handleClick,
      stopPropagation,
      observer: this._observer,
    });
    const links = buildLinks(nodes);
    const rootSourceRect = sourceRectMap[rootNode.state.id];
    const { mmPathExit, mmPathMerge } = renderLinks({
      g: this.g,
      linkSelector: SELECTOR_LINK,
      links,
      originMap,
      sourceRectMap,
      rootRect: rootSourceRect,
      linkShape,
    });
    await new Promise(requestAnimationFrame);
    measureNodeSizes(this.g, SELECTOR_NODE);
    const { rects, bounds } = computeLayout(rootNode, this.options);
    applyRectsToNodes(rects, nodes);
    this.state.rect = bounds;
    const rootTargetRect = rootNode.state.rect;

    const updatedHighlightRect = highlightVisible
      ? buildHighlightRect(highlightVisible, rootNode, transform)
      : undefined;
    animateEnterExit({
      duration: this.options.duration,
      highlightNodes,
      highlightRect: updatedHighlightRect,
      mmGEnter,
      mmGExit,
      mmGMerge,
      mmCircleMerge,
      mmCircleTextMerge,
      mmCircleTextExit,
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
    });

    if (autoFit) this.fit();
  }

  transition<T extends d3.BaseType, U, P extends d3.BaseType, Q>(
    sel: d3.Selection<T, U, P, Q>,
  ): d3.Transition<T, U, P, Q> {
    const { duration } = this.options;
    return sel.transition().duration(duration);
  }

  /**
   * Fit the content to the viewport.
   */
  async fit(maxScale = this.options.maxInitialScale): Promise<void> {
    const svgNode = this.svg.node()!;
    const { width: offsetWidth, height: offsetHeight } =
      svgNode.getBoundingClientRect();
    const { fitRatio } = this.options;
    const { x1, y1, x2, y2 } = this.state.rect;
    const naturalWidth = x2 - x1;
    const naturalHeight = y2 - y1;
    const scale = Math.min(
      (offsetWidth / naturalWidth) * fitRatio,
      (offsetHeight / naturalHeight) * fitRatio,
      maxScale,
    );
    const initialZoom = zoomIdentity
      .translate(
        (offsetWidth - naturalWidth * scale) / 2 - x1 * scale,
        (offsetHeight - naturalHeight * scale) / 2 - y1 * scale,
      )
      .scale(scale);
    return this.transition(this.svg)
      .call(this.zoom.transform, initialZoom)
      .end()
      .catch(noop);
  }

  findElement(node: INode) {
    let result:
      | {
          data: INode;
          g: SVGGElement;
        }
      | undefined;
    this.g
      .selectAll<SVGGElement, INode>(childSelector<SVGGElement>(SELECTOR_NODE))
      .each(function walk(d) {
        if (d === node) {
          result = {
            data: d,
            g: this,
          };
        }
      });
    return result;
  }

  /**
   * Pan the content to make the provided node visible in the viewport.
   */
  async ensureVisible(node: INode, padding?: Partial<IPadding>) {
    const itemData = this.findElement(node)?.data;
    if (!itemData) return;
    const svgNode = this.svg.node()!;
    const relRect = svgNode.getBoundingClientRect();
    const transform = zoomTransform(svgNode);
    const [left, right] = [
      itemData.state.rect.x,
      itemData.state.rect.x + itemData.state.rect.width + 2,
    ].map((x) => x * transform.k + transform.x);
    const [top, bottom] = [
      itemData.state.rect.y,
      itemData.state.rect.y + itemData.state.rect.height,
    ].map((y) => y * transform.k + transform.y);
    // Skip if the node includes or is included in the container.
    const pd: IPadding = {
      left: 0,
      right: 0,
      top: 0,
      bottom: 0,
      ...padding,
    };
    const dxs = [pd.left - left, relRect.width - pd.right - right];
    const dys = [pd.top - top, relRect.height - pd.bottom - bottom];
    const dx = dxs[0] * dxs[1] > 0 ? minBy(dxs, Math.abs) / transform.k : 0;
    const dy = dys[0] * dys[1] > 0 ? minBy(dys, Math.abs) / transform.k : 0;
    if (dx || dy) {
      const newTransform = transform.translate(dx, dy);
      return this.transition(this.svg)
        .call(this.zoom.transform, newTransform)
        .end()
        .catch(noop);
    }
  }

  /** @deprecated Use `ensureVisible` instead */
  ensureView = this.ensureVisible;

  async centerNode(node: INode, padding?: Partial<IPadding>) {
    const itemData = this.findElement(node)?.data;
    if (!itemData) return;
    const svgNode = this.svg.node()!;
    const relRect = svgNode.getBoundingClientRect();
    const transform = zoomTransform(svgNode);
    const x =
      (itemData.state.rect.x + itemData.state.rect.width / 2) * transform.k +
      transform.x;
    const y =
      (itemData.state.rect.y + itemData.state.rect.height / 2) * transform.k +
      transform.y;
    const pd: IPadding = {
      left: 0,
      right: 0,
      top: 0,
      bottom: 0,
      ...padding,
    };
    const cx = (pd.left + relRect.width - pd.right) / 2;
    const cy = (pd.top + relRect.height - pd.bottom) / 2;
    const dx = (cx - x) / transform.k;
    const dy = (cy - y) / transform.k;
    if (dx || dy) {
      const newTransform = transform.translate(dx, dy);
      return this.transition(this.svg)
        .call(this.zoom.transform, newTransform)
        .end()
        .catch(noop);
    }
  }

  /**
   * Scale content with it pinned at the center of the viewport.
   */
  async rescale(scale: number): Promise<void> {
    const svgNode = this.svg.node()!;
    const { width: offsetWidth, height: offsetHeight } =
      svgNode.getBoundingClientRect();
    const halfWidth = offsetWidth / 2;
    const halfHeight = offsetHeight / 2;
    const transform = zoomTransform(svgNode);
    const newTransform = transform
      .translate(
        ((halfWidth - transform.x) * (1 - scale)) / transform.k,
        ((halfHeight - transform.y) * (1 - scale)) / transform.k,
      )
      .scale(scale);
    return this.transition(this.svg)
      .call(this.zoom.transform, newTransform)
      .end()
      .catch(noop);
  }

  destroy() {
    this.svg.on('.zoom', null);
    this.svg.html(null);
    this._disposeList.forEach((fn) => {
      fn();
    });
  }

  static create(
    svg: string | SVGElement | ID3SVGElement,
    opts?: Partial<IMarkmapOptions>,
    data: IPureNode | null = null,
  ): Markmap {
    const mm = new Markmap(svg, opts);
    if (data) {
      mm.setData(data).then(() => {
        mm.fit(); // always fit for the first render
      });
    }
    return mm;
  }
}
