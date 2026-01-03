import type { FileTreeNode } from "./types";
import type { StateIconState } from "../../../ui/icons/state/types";

export const resolveFileNodeState = (
  node: FileTreeNode,
  isExpanded?: boolean
): StateIconState => {
  if (node.type === "file") return "file";
  return isExpanded ? "folder-open" : "folder-closed";
};
