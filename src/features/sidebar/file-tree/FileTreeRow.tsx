import type { FileTreeNode } from "./types";
import { renderIcon } from "./renderIcon";

export type FileTreeRowProps = {
  node: FileTreeNode;
};

export const FileTreeRow = (props: FileTreeRowProps) => {
  return (
    <span class="file-tree-row-content">
      <span class="file-tree-icon" aria-hidden="true">
        {renderIcon(props.node)}
      </span>
      <span class="file-tree-label">{props.node.name}</span>
    </span>
  );
};
