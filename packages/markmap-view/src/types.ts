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
   * Render a custom editor instead of the default contenteditable behavior.
   * This is useful for integrating rich editors like CodeMirror or Monaco.
   */
  renderEditor?: (args: IEditorArgs) => void;
}

export interface IEditorArgs {
  /** The node being edited */
  node: INode;
  /** The DOM rect of the node's content element (for positioning) */
  rect: DOMRect;
  /** The initial text content */
  initialContent: string;
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
