import type { FileTreeNode } from "./types";
import { renderIcon } from "./renderIcon";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";

export type FileTreeRowProps = {
  node: FileTreeNode;
  style?: FileTreeStyle;
};

export const FileTreeRow = (props: FileTreeRowProps) => {
  return (
    <span class="file-tree-row-content">
      <span class="file-tree-icon" aria-hidden="true">
        {renderIcon(props.node, props.style)}
      </span>
      <span class="file-tree-label">{props.node.name}</span>
    </span>
  );
};
