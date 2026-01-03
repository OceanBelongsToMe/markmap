import { TreeView } from "@ark-ui/solid/tree-view";
import type { JSX } from "solid-js";

export type FileTreeBranchControlProps = {
  depth: number;
  children: JSX.Element;
};

export const FileTreeBranchControl = (props: FileTreeBranchControlProps) => {
  return (
    <TreeView.BranchTrigger
      class="file-tree-branch-trigger"
      style={{ "--depth": `${props.depth}` }}
    >
      {props.children}
    </TreeView.BranchTrigger>
  );
};
