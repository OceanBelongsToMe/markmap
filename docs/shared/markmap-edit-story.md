# Markmap Subtree Edit Story (Frontend AST → Backend Update)

## Goal
Enable subtree editing with **stable node IDs** ("地狱级稳定") without inserting any visible markers into the Markdown editor. The solution uses CodeMirror syntax tree extraction, hidden node-id anchors, and backend incremental updates.

## Constraints & Decisions
- Block/inline split follows `MarkdownKind` definitions.
- NodeRange is optional; ignore if unavailable.
- Node type changes can reuse the same node_id.
- **No visible id markers** in the Markdown text.
- Parsing is done on the frontend via CodeMirror syntax tree.

## High-Level Flow
1. **Edit start**: Load subtree markdown and node-id anchors.
2. **Edit session**: CodeMirror maintains hidden anchors (block/inline).
3. **Save**: Frontend extracts AST + binds node-id anchors → ResolvedAst.
4. **Backend**: Apply incremental update using ResolvedAst.

## Data Structures
### Frontend AST
```ts
type AstNode = {
  kind: MarkdownKind;
  text?: string;
  children: AstNode[];
  from?: number;
  to?: number;
};

type ResolvedAstNode = AstNode & {
  nodeId?: string;
};

type ResolvedAst = {
  root: ResolvedAstNode;
};
```

### NodeId Anchors (editor metadata)
```ts
type NodeIdAnchor =
  | { kind: "block"; line: number; nodeId: string }
  | { kind: "inline"; from: number; to: number; nodeId: string };
```

## SRP & DIP Boundaries
### Frontend
- **AstExtractor**: syntax tree → AstNode
- **NodeIdBinder**: anchors → ResolvedAst
- **AstSerializer**: ResolvedAst → DTO
- **EditUseCase**: orchestrates save

### Backend
- **AstUpdater**: ResolvedAst → incremental update
- **NodeRepository**: persistence only
- **NodeCleaner**: deletes removed nodes

## Binding Rules (Stable IDs)
1. **Block nodes**: bind by line anchor.
2. **Inline nodes**: bind by inline mark overlap.
3. No match → no nodeId (backend generates).

## Incremental Update (Backend)
- Traverse ResolvedAst:
  - If nodeId present → update existing node
  - If nodeId missing → create new node
- Build parent/child relations from AST tree
- Delete nodes in old subtree not present in ResolvedAst

## API Payload (Frontend → Backend)
```json
{
  "document_id": "...",
  "root_node_id": "...",
  "ast": {
    "root": {
      "kind": "heading",
      "node_id": "UUID",
      "children": [
        { "kind": "text", "node_id": "UUID", "text": "Hello" }
      ]
    }
  }
}
```

## Open Questions
- Which inline nodes should be anchored (all inline kinds or just text/link/emphasis)?
- Should backend generate missing ids or return validation error?

