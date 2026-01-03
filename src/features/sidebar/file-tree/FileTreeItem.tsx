import { TreeView } from "@ark-ui/solid/tree-view";
import { ChevronIcon } from "../../../ui/components/ChevronIcon";
import { FileTreeRow } from "./FileTreeRow";
import type { FlatFileTreeNode } from "./flattenFileTree";

export type FileTreeItemProps = {
  entry: FlatFileTreeNode;
};

export const FileTreeItem = (props: FileTreeItemProps) => {
  const { node, depth, isFolder, indexPath } = props.entry;
  return (
    <TreeView.NodeProvider node={node} indexPath={indexPath}>
      {isFolder ? (
        <TreeView.Branch>
          <TreeView.BranchControl style={{ "--depth": `${depth}` }}>
          <TreeView.BranchText>
            <FileTreeRow node={node} />
          </TreeView.BranchText>
          <TreeView.BranchIndicator
            class="file-tree-caret collapsible-chevron"
            aria-hidden="true"
          >
            <ChevronIcon class="collapsible-chevron-icon" />
          </TreeView.BranchIndicator>
        </TreeView.BranchControl>
      </TreeView.Branch>
      ) : (
        <TreeView.Item style={{ "--depth": `${depth}` }}>
          <TreeView.ItemText>
            <FileTreeRow node={node} />
          </TreeView.ItemText>
        </TreeView.Item>
      )}
    </TreeView.NodeProvider>
  );
};
