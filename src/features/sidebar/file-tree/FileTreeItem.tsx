import { TreeView } from "@ark-ui/solid/tree-view";
import { FileTreeRow } from "./FileTreeRow";
import type { FlatFileTreeNode } from "./flattenFileTree";
import type { FileTreeStyle } from "./style/fileTreeStyleTypes";
import { StateIcon } from "../../../ui/icons/state/StateIcon";
import { FileTreeBranchControl } from "./FileTreeBranchControl";
import { FileTreeBranchContent } from "./FileTreeBranchContent";
import { resolveBranchState } from "./resolveBranchState";

export type FileTreeItemProps = {
  entry: FlatFileTreeNode;
  style?: FileTreeStyle;
};

export const FileTreeItem = (props: FileTreeItemProps) => {
  const { node, depth, isFolder, isExpanded, indexPath } = props.entry;
  return (
    <TreeView.NodeProvider node={node} indexPath={indexPath}>
      {isFolder ? (
        <TreeView.Branch>
          <TreeView.NodeContext>
            {(ctx) => (
              <FileTreeBranchControl depth={depth}>
                <FileTreeBranchContent
                  content={
                    <FileTreeRow
                      node={node}
                      style={props.style}
                      isExpanded={ctx().expanded}
                    />
                  }
                  trailing={
                    <span class="file-tree-branch-caret collapsible-chevron">
                      <StateIcon
                        context="collapsible"
                        state={resolveBranchState(ctx().expanded)}
                      />
                    </span>
                  }
                />
              </FileTreeBranchControl>
            )}
          </TreeView.NodeContext>
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
