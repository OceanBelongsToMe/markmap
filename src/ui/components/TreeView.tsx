import { StableList } from "./StableList";

export type TreeNode = {
  id: string;
  label: string;
  children?: TreeNode[];
};

export type TreeViewProps = {
  nodes: TreeNode[];
};

export const TreeView = (props: TreeViewProps) => {
  return (
    <ul>
      <StableList each={() => props.nodes}>
        {(node) => (
          <li>
            {node().label}
            {node().children ? <TreeView nodes={node().children ?? []} /> : null}
          </li>
        )}
      </StableList>
    </ul>
  );
};
