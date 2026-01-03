import { TreeView } from "@ark-ui/solid/tree-view";
import { ChevronRight } from "lucide-solid";
import { FileTreeRow } from "./FileTreeRow";
import type { FlatFileTreeNode } from "./flattenFileTree";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";

export type FileTreeItemProps = {
  entry: FlatFileTreeNode;
  style?: FileTreeStyle;
};

export const FileTreeItem = (props: FileTreeItemProps) => {
  const { node, depth, isFolder, indexPath } = props.entry;
  return (
    <TreeView.NodeProvider node={node} indexPath={indexPath}>
      {isFolder ? (
        <TreeView.Branch>
          <TreeView.BranchControl style={{ "--depth": `${depth}` }}>
          <TreeView.BranchText>
            <FileTreeRow node={node} style={props.style} />
          </TreeView.BranchText>
          <TreeView.BranchIndicator
            class="file-tree-caret collapsible-chevron"
            aria-hidden="true"
          >
            <ChevronRight class="collapsible-chevron-icon" />
          </TreeView.BranchIndicator>
        </TreeView.BranchControl>
      </TreeView.Branch>
      ) : (
        <TreeView.Item style={{ "--depth": `${depth}` }}>
          <TreeView.ItemText>
            <FileTreeRow node={node} style={props.style} />
          </TreeView.ItemText>
        </TreeView.Item>
      )}
    </TreeView.NodeProvider>
  );
};
