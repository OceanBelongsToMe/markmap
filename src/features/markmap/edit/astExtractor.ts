import type { SyntaxNode, Tree } from "@lezer/common";
import type { AstNode, MarkdownKind } from "./types";

const BLOCK_KIND_BY_NAME: Record<string, MarkdownKind> = {
  ATXHeading: "heading",
  SetextHeading: "heading",
  List: "list",
  ListItem: "list_item",
  Paragraph: "paragraph",
  Blockquote: "blockquote",
  FencedCode: "code_block",
  CodeBlock: "code_block",
  HTMLBlock: "html_block",
  Table: "table",
  ThematicBreak: "thematic_break",
};

const INLINE_KIND_BY_NAME: Record<string, MarkdownKind> = {
  Text: "text",
  Emphasis: "emphasis",
  StrongEmphasis: "strong",
  Strikethrough: "strikethrough",
  Superscript: "superscript",
  Subscript: "subscript",
  InlineCode: "inline_code",
  Link: "link",
  Image: "image",
  HTMLTag: "html_inline",
};

function mapKind(node: SyntaxNode): MarkdownKind | null {
  return BLOCK_KIND_BY_NAME[node.name] || INLINE_KIND_BY_NAME[node.name] || null;
}

function isBlockKind(kind: MarkdownKind): boolean {
  return (
    kind === "heading" ||
    kind === "list" ||
    kind === "list_item" ||
    kind === "paragraph" ||
    kind === "blockquote" ||
    kind === "code_block" ||
    kind === "table" ||
    kind === "html_block" ||
    kind === "thematic_break"
  );
}

function isInlineKind(kind: MarkdownKind): boolean {
  return !isBlockKind(kind) && kind !== "unknown";
}

function buildNode(node: SyntaxNode, doc: string): AstNode | null {
  const kind = mapKind(node) || "unknown";
  const children: AstNode[] = [];

  for (let child = node.firstChild; child; child = child.nextSibling) {
    const childKind = mapKind(child);
    if (!childKind) {
      const nested = buildNode(child, doc);
      if (nested) children.push(nested);
      continue;
    }
    const nested = buildNode(child, doc);
    if (nested) children.push(nested);
  }

  const astNode: AstNode = {
    kind,
    from: node.from,
    to: node.to,
    children,
  };

  if (isInlineKind(kind)) {
    astNode.text = doc.slice(node.from, node.to);
  }

  return astNode;
}

export function extractMarkdownAst(doc: string, tree: Tree): AstNode[] {
  const nodes: AstNode[] = [];
  let cursor = tree.cursor();
  for (let child = cursor.firstChild(); child; child = cursor.nextSibling()) {
    const node = cursor.node;
    const kind = mapKind(node);
    if (!kind) continue;
    const astNode = buildNode(node, doc);
    if (!astNode) continue;
    if (isBlockKind(astNode.kind) || isInlineKind(astNode.kind)) {
      nodes.push(astNode);
    }
  }
  return nodes;
}
