import type { JSX } from "solid-js";
import type { FileTreeNode } from "./useFileTreeData";

export type FileTreeViewProps = {
  nodes: FileTreeNode[];
  selectedId?: string | null;
  onSelect?: (id: string) => void;
  renderLabel?: (node: FileTreeNode) => JSX.Element;
};

export const FileTreeView = (props: FileTreeViewProps) => {
  return (
    <div>
      {props.nodes.map((node) => (
        <button type="button" onClick={() => props.onSelect?.(node.id)}>
          {props.renderLabel ? props.renderLabel(node) : node.label}
        </button>
      ))}
    </div>
  );
};
