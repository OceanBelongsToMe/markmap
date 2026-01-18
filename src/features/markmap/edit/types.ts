export type MarkdownKind =
  | "heading"
  | "list"
  | "list_item"
  | "paragraph"
  | "blockquote"
  | "code_block"
  | "table"
  | "text"
  | "emphasis"
  | "strong"
  | "strikethrough"
  | "superscript"
  | "subscript"
  | "inline_code"
  | "link"
  | "image"
  | "html_inline"
  | "html_block"
  | "thematic_break"
  | "unknown";

export type AstNode = {
  kind: MarkdownKind;
  from: number;
  to: number;
  text?: string;
  children: AstNode[];
};

export type ResolvedAstNode = AstNode & {
  nodeId?: string;
};

export type ResolvedAst = {
  root: ResolvedAstNode;
};

export type NodeIdAnchor =
  | { kind: "block"; line: number; nodeId: string }
  | { kind: "inline"; from: number; to: number; nodeId: string };
