import type { FileTreeNode } from "../../file-tree/domain/types";
import type { WorkspaceRecentFile } from "../../../../state/workspace/useRecentFiles";
import type {
  WorkspaceDocumentNode,
  WorkspaceFileTree
} from "../../../../state/workspace/useWorkspaceTree";

export type RecentFileEntry = {
  documentId: string;
  lastOpenedAt: number;
  node: FileTreeNode;
};

const baseName = (value: string) => {
  const trimmed = value.replace(/\/+$/, "");
  const parts = trimmed.split(/[\\/]/).filter(Boolean);
  return parts[parts.length - 1] ?? value;
};

const buildDocumentIndex = (tree: WorkspaceFileTree | null) => {
  const map = new Map<string, WorkspaceDocumentNode>();
  if (!tree) return map;
  for (const folder of tree.folders) {
    for (const doc of folder.documents) {
      map.set(doc.id, doc);
    }
  }
  return map;
};

export const mapRecentFilesToEntries = (
  items: WorkspaceRecentFile[],
  tree: WorkspaceFileTree | null
): RecentFileEntry[] => {
  if (!tree) return [];
  const docIndex = buildDocumentIndex(tree);
  const entries: RecentFileEntry[] = [];
  for (const item of items) {
    const doc = docIndex.get(item.documentId);
    if (!doc) continue;
    entries.push({
      documentId: item.documentId,
      lastOpenedAt: item.lastOpenedAt,
      node: {
        id: doc.id,
        name: baseName(doc.path),
        type: "file"
      }
    });
  }
  return entries;
};
