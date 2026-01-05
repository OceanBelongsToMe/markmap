import type { JSX } from "solid-js";
import type { FileTreeNode } from "../domain/types";
import { renderIcon } from "./renderIcon";
import type { FileTreeStyle } from "../style/fileTreeStyleTypes";

export type FileTreeRowProps = {
  node: FileTreeNode;
  style?: FileTreeStyle;
  isExpanded?: boolean;
  suffix?: JSX.Element;
};

export const FileTreeRow = (props: FileTreeRowProps) => {
  return (
    <span class="file-tree-row-content">
      <span class="file-tree-icon" aria-hidden="true">
        {renderIcon(props.node, props.style, props.isExpanded)}
      </span>
      <span class="file-tree-label">{props.node.name}</span>
      {props.suffix ? (
        <span class="file-tree-row-suffix" aria-hidden="true">
          {props.suffix}
        </span>
      ) : null}
    </span>
  );
};
