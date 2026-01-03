import type { JSX } from "solid-js";
import { ArkTreeView } from "../ark/tree-view/TreeView";

export type TreeNode = {
  id: string;
  label: string;
  children?: TreeNode[];
};

export type TreeViewProps = {
  nodes: TreeNode[];
  selectedId?: string;
  expandedIds?: string[];
  density?: "comfortable" | "compact";
  class?: string;
  ariaLabel?: string;
  onSelect?: (id: string) => void;
  onExpandedChange?: (ids: string[]) => void;
  renderLabel?: (node: TreeNode) => JSX.Element;
};

export const TreeView = (props: TreeViewProps) => {
  const densityClass =
    props.density === "compact" ? "ui-tree-view--compact" : "ui-tree-view--comfortable";
  const className = ["ui-tree-view", densityClass, props.class].filter(Boolean).join(" ");

  return (
    <ArkTreeView
      items={props.nodes}
      class={className}
      ariaLabel={props.ariaLabel}
      selectedId={props.selectedId}
      onSelect={props.onSelect}
      expandedIds={props.expandedIds}
      onExpandedChange={props.onExpandedChange}
      renderLabel={props.renderLabel}
    />
  );
};
