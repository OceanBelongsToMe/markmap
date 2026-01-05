import type { JSX } from "solid-js";

export type FileTreeBranchContentProps = {
  leading?: JSX.Element;
  content: JSX.Element;
  trailing: JSX.Element;
};

export const FileTreeBranchContent = (props: FileTreeBranchContentProps) => {
  return (
    <>
      {props.leading ? (
        <span class="file-tree-branch-leading">{props.leading}</span>
      ) : null}
      <span class="file-tree-branch-content">{props.content}</span>
      <span class="file-tree-branch-trailing">{props.trailing}</span>
    </>
  );
};
