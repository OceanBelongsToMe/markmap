import type { WorkspaceFileTree } from "../../../../state/workspace/useWorkspaceTree";
import type { FileTreeNode } from "../domain/types";
import { buildPathTree } from "../domain/buildPathTree";

const baseName = (value: string) => {
  const trimmed = value.replace(/\/+$/, "");
  const parts = trimmed.split(/[\\/]/).filter(Boolean);
  return parts[parts.length - 1] ?? value;
};

export const mapWorkspaceTreeToFileNodes = (tree: WorkspaceFileTree | null): FileTreeNode[] => {
  if (!tree) return [];
  return tree.folders.map((folder) => ({
    id: folder.id,
    name: baseName(folder.rootPath),
    type: "folder",
    children: buildPathTree(
      folder.id,
      folder.documents.map((doc) => ({
        id: doc.id,
        path: doc.path
      }))
    )
  }));
};
