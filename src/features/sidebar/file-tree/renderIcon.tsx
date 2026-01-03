import type { FileTreeIcon, FileTreeNode } from "./types";

export const renderIcon = (node: FileTreeNode) => {
  const icon = resolveIcon(node);
  if (icon.kind === "image") {
    return <img src={icon.src} alt={icon.alt ?? ""} />;
  }
  return icon.value;
};

const resolveIcon = (node: FileTreeNode): FileTreeIcon => {
  if (node.icon) return node.icon;
  if (node.type === "folder") {
    return { kind: "emoji", value: "📁" };
  }
  return { kind: "emoji", value: "📄" };
};
