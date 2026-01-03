import { FileText, Folder } from "lucide-solid";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";
import type { FileTreeIcon, FileTreeNode } from "./types";

export const renderIcon = (node: FileTreeNode, style?: FileTreeStyle) => {
  const icon = resolveIcon(node, style);
  if (icon.kind === "image") {
    return <img src={icon.src} alt={icon.alt ?? ""} />;
  }
  if (icon.kind === "emoji") {
    return icon.value;
  }
  return icon.node;
};

const resolveIcon = (node: FileTreeNode, style?: FileTreeStyle): FileTreeIcon => {
  if (node.icon) return node.icon;
  const useLucide = style !== "classic";
  if (useLucide) {
    if (node.type === "folder") {
      return {
        kind: "lucide",
        node: <Folder class="file-tree-icon-svg" />
      };
    }
    return { kind: "lucide", node: <FileText class="file-tree-icon-svg" /> };
  }
  if (node.type === "folder") {
    return { kind: "emoji", value: "📁" };
  }
  return { kind: "emoji", value: "📄" };
};
