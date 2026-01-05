import type { FileTreeStyle } from "../style/fileTreeStyleTypes";
import type { FileTreeIcon, FileTreeNode } from "../domain/types";
import { StateIcon } from "../../../../ui/icons/state/StateIcon";
import { resolveFileNodeState } from "./resolveFileNodeState";

export const renderIcon = (
  node: FileTreeNode,
  style?: FileTreeStyle,
  isExpanded?: boolean
) => {
  const icon = resolveIcon(node, style, isExpanded);
  if (icon.kind === "image") {
    return <img src={icon.src} alt={icon.alt ?? ""} />;
  }
  if (icon.kind === "emoji") {
    return <span class="file-tree-emoji">{icon.value}</span>;
  }
  return icon.node;
};

const resolveIcon = (
  node: FileTreeNode,
  style?: FileTreeStyle,
  isExpanded?: boolean
): FileTreeIcon => {
  if (node.icon) return node.icon;
  return {
    kind: "lucide",
    node: (
      <StateIcon
        context="file-node"
        state={resolveFileNodeState(node, isExpanded)}
        theme={style}
      />
    )
  };
};
