import { INode } from 'markmap-common';

export interface IMarkmapState {
  id: string;
  data?: INode;
  highlight?: INode;
  rect: {
    x1: number;
    x2: number;
    y1: number;
    y2: number;
  };
}

export interface INodeLoader {
  loadChildren: (nodeId: string) => Promise<INode[]>;
}

/**
 * Portable options that can be derived into `IMarkmapOptions`.
 */
export interface IMarkmapJSONOptions {
  color: string[];
  colorFreezeLevel: number;
  duration: number;
  extraCss: string[];
  extraJs: string[];
  fitRatio: number;
  initialExpandLevel: number;
  maxInitialScale: number;
  maxWidth: number;
  nodeMinHeight: number;
  paddingX: number;
  pan: boolean;
  spacingHorizontal: number;
  spacingVertical: number;
  zoom: boolean;
  lineWidth: number | number[];
}

export interface IMarkmapOptions {
  autoFit: boolean;
  duration: number;
  embedGlobalCSS: boolean;
  fitRatio: number;
  id?: string;
  initialExpandLevel: number;
  maxInitialScale: number;
  pan: boolean;
  scrollForPan: boolean;
  style?: (id: string) => string;
  toggleRecursively: boolean;
  zoom: boolean;

  // Theme options
  color: (node: INode) => string;
  lineWidth: (node: INode) => number;
  maxWidth: number;
  nodeMinHeight: number;
  paddingX: number;
  spacingHorizontal: number;
  spacingVertical: number;

  // Node content rendering
  nodeContent?: (node: INode) => string;

  // Inline editing
  editable?: IMarkmapEditableOptions;
}

export interface IMarkmapEditableOptions {
  enabled?: boolean;
  getNodeId?: (node: INode) => string | number | undefined;
  onCommit?: (
    nodeId: string | number,
    text: string,
    node: INode,
  ) => void | Promise<void>;
  onCancel?: (nodeId: string | number, node: INode) => void;
  /**
   * Provide a custom editor implementation.
   * If not set, the default contenteditable editor will be used.
   */
  editor?: IInlineEditorAdapter;
}

export interface IInlineEditorAdapter {
  open: (args: IEditorArgs) => IInlineEditorSession | void;
  /**
   * Whether to temporarily disable SVG pointer events while editing.
   */
  lockPointerEvents?: boolean;
}

export interface IInlineEditorSession {
  close: (opts?: { cancel?: boolean }) => void;
}

export interface IEditorArgs {
  /** The node being edited */
  node: INode;
  /** The DOM rect of the node's content element (for positioning) */
  rect: DOMRect;
  /** The current zoom scale of the SVG */
  k: number;
  /** The horizontal padding of the node (from markmap options) */
  paddingX: number;
  /** The initial text content */
  initialContent: string;
  /** The content container inside foreignObject */
  host?: HTMLDivElement;
  /** The foreignObject element of the node */
  foreignObject?: SVGForeignObjectElement;
  /** Callback to commit changes */
  save: (newContent: string) => void;
  /** Callback to cancel editing */
  cancel: () => void;
}

export interface IPadding {
  left: number;
  right: number;
  top: number;
  bottom: number;
}

export type ID3SVGElement = d3.Selection<SVGElement, INode, HTMLElement, INode>;
