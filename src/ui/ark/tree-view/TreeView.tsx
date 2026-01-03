import type { JSX } from "solid-js";
import { For, Show, createMemo } from "solid-js";
import { TreeView, createTreeCollection } from "@ark-ui/solid/tree-view";

export type ArkTreeViewItem = {
  id: string;
  label: string;
  children?: ArkTreeViewItem[];
};

export type ArkTreeViewProps = {
  items: ArkTreeViewItem[];
  class?: string;
  ariaLabel?: string;
  selectedId?: string;
  expandedIds?: string[];
  onSelect?: (id: string) => void;
  onExpandedChange?: (ids: string[]) => void;
  renderLabel?: (item: ArkTreeViewItem) => JSX.Element;
};

type TreeNodeProps = TreeView.NodeProviderProps<ArkTreeViewItem> & {
  renderLabel?: (item: ArkTreeViewItem) => JSX.Element;
};

const TreeNode = (props: TreeNodeProps) => {
  const { node, indexPath } = props;
  const label = props.renderLabel ? props.renderLabel(node) : node.label;
  return (
    <TreeView.NodeProvider node={node} indexPath={indexPath}>
      <Show
        when={node.children?.length}
        fallback={
          <TreeView.Item>
            <TreeView.ItemIndicator aria-hidden="true" />
            <TreeView.ItemText>{label}</TreeView.ItemText>
          </TreeView.Item>
        }
      >
        <TreeView.Branch>
          <TreeView.BranchControl>
            <TreeView.BranchText>{label}</TreeView.BranchText>
            <TreeView.BranchIndicator aria-hidden="true" />
          </TreeView.BranchControl>
          <TreeView.BranchContent>
            <TreeView.BranchIndentGuide />
            <For each={node.children}>
              {(child, index) => (
                <TreeNode
                  node={child}
                  indexPath={[...indexPath, index()]}
                  renderLabel={props.renderLabel}
                />
              )}
            </For>
          </TreeView.BranchContent>
        </TreeView.Branch>
      </Show>
    </TreeView.NodeProvider>
  );
};

export const ArkTreeView = (props: ArkTreeViewProps) => {
  const collection = createMemo(() =>
    createTreeCollection<ArkTreeViewItem>({
      nodeToValue: (node) => node.id,
      nodeToString: (node) => node.label,
      rootNode: {
        id: "ROOT",
        label: "",
        children: props.items
      }
    })
  );
  const selectedValue = () => (props.selectedId ? [props.selectedId] : undefined);
  const expandedValue = () => props.expandedIds;
  const handleSelectionChange = (details: { selectedValue: string[] }) => {
    if (!props.onSelect) return;
    const next = details.selectedValue[0];
    if (next) {
      props.onSelect(next);
    }
  };
  const handleExpandedChange = (details: { expandedValue: string[] }) => {
    props.onExpandedChange?.(details.expandedValue);
  };

  return (
    <TreeView.Root
      class={props.class}
      collection={collection()}
      aria-label={props.ariaLabel}
      selectedValue={selectedValue()}
      onSelectionChange={props.onSelect ? handleSelectionChange : undefined}
      expandedValue={expandedValue()}
      onExpandedChange={props.onExpandedChange ? handleExpandedChange : undefined}
    >
      <TreeView.Tree>
        <For each={collection().rootNode.children ?? []}>
          {(node, index) => (
            <TreeNode
              node={node}
              indexPath={[index()]}
              renderLabel={props.renderLabel}
            />
          )}
        </For>
      </TreeView.Tree>
    </TreeView.Root>
  );
};
