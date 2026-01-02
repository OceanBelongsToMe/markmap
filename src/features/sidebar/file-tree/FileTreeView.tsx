import type { Accessor } from "solid-js";
import { createMemo, Show } from "solid-js";
import { ChevronIcon } from "../../../ui/components/ChevronIcon";
import { StableList } from "../../../ui/components/StableList";
import { useCollapsible } from "../../../ui/components/useCollapsible";
import type { FileTreeIcon, FileTreeNode } from "./types";
import "./fileTree.css";

export type FileTreeViewProps = {
  nodes: Accessor<FileTreeNode[]>;
  expandedIds: Accessor<string[]>;
  selectedId?: Accessor<string | null>;
  onNodeClick?: (node: FileTreeNode) => void;
};

export const FileTreeView = (props: FileTreeViewProps) => {
  return (
    <ul class="file-tree" role="tree">
      <StableList each={props.nodes}>
        {(node) => (
          <FileTreeNodeRow
            node={node}
            depth={0}
            expandedIds={props.expandedIds}
            selectedId={props.selectedId}
            onNodeClick={props.onNodeClick}
          />
        )}
      </StableList>
    </ul>
  );
};

type FileTreeNodeRowProps = {
  node: Accessor<FileTreeNode>;
  depth: number;
  expandedIds: Accessor<string[]>;
  selectedId?: Accessor<string | null>;
  onNodeClick?: (node: FileTreeNode) => void;
};

const FileTreeNodeRow = (props: FileTreeNodeRowProps) => {
  const isFolder = createMemo(() => props.node().type === "folder");
  const isExpanded = createMemo(() =>
    props.expandedIds().includes(props.node().id)
  );
  const isSelected = createMemo(() => props.selectedId?.() === props.node().id);
  const { isOpen, isCollapsed } = useCollapsible(isExpanded);

  const onClickRow = () => {
    props.onNodeClick?.(props.node());
  };

  return (
    <li
      class="file-tree-node collapsible"
      data-type={props.node().type}
      data-selected={isSelected()}
      data-collapsed={isFolder() ? (isCollapsed() ? "true" : "false") : undefined}
      role="treeitem"
      aria-expanded={isFolder() ? isOpen() : undefined}
      aria-selected={isSelected()}
    >
      <button
        type="button"
        class="file-tree-row collapsible-trigger"
        style={{ "--depth": `${props.depth}` }}
        onClick={onClickRow}
        data-collapsed={isFolder() ? (isCollapsed() ? "true" : "false") : undefined}
      >
        <span class="file-tree-caret collapsible-chevron" aria-hidden="true">
          <ChevronIcon class="collapsible-chevron-icon" />
        </span>
        <span class="file-tree-icon" aria-hidden="true">
          {renderIcon(props.node())}
        </span>
        <span class="file-tree-label">{props.node().name}</span>
      </button>
      <Show when={isFolder()}>
        <div class="file-tree-children collapsible-body">
          <ul role="group">
            <StableList each={() => props.node().children ?? []}>
              {(child) => (
                <FileTreeNodeRow
                  node={child}
                  depth={props.depth + 1}
                  expandedIds={props.expandedIds}
                  selectedId={props.selectedId}
                  onNodeClick={props.onNodeClick}
                />
              )}
            </StableList>
          </ul>
        </div>
      </Show>
    </li>
  );
};

const renderIcon = (node: FileTreeNode) => {
  const icon = resolveIcon(node);
  if (icon.kind === "image") {
    return <img src={icon.src} alt={icon.alt ?? ""} />;
  }
  return icon.value;
};

const resolveIcon = (node: FileTreeNode): FileTreeIcon => {
  if (node.icon) return node.icon;
  if (node.type === "folder") {
    return { kind: "emoji", value: "📁" };
  }
  return { kind: "emoji", value: "📄" };
};
