import type { Accessor } from "solid-js";
import { createMemo } from "solid-js";
import { TreeView, createTreeCollection } from "@ark-ui/solid/tree-view";
import { flattenFileTree } from "./flattenFileTree";
import type { FileTreeNode } from "./types";
import { VirtualTreeList } from "../../../ui/virtual/VirtualTreeList";
import { FileTreeItem } from "./FileTreeItem";
import "./fileTree.css";

export type FileTreeViewProps = {
  nodes: Accessor<FileTreeNode[]>;
  expandedIds: Accessor<string[]>;
  selectedId?: Accessor<string | null>;
  ariaLabel?: string;
  onSelect?: (id: string) => void;
  onExpandedChange?: (ids: string[]) => void;
};

const ROW_HEIGHT = 28;

export const FileTreeView = (props: FileTreeViewProps) => {
  const flatNodes = createMemo(() =>
    flattenFileTree(props.nodes(), props.expandedIds())
  );
  const flatIds = createMemo(() => flatNodes().map((node) => node.node.id));
  const entryMap = createMemo(() => {
    const map = new Map<string, ReturnType<typeof flatNodes>[number]>();
    for (const entry of flatNodes()) {
      map.set(entry.node.id, entry);
    }
    return map;
  });

  const collection = createMemo(() =>
    createTreeCollection<FileTreeNode>({
      nodeToValue: (node) => node.id,
      nodeToString: (node) => node.name,
      rootNode: {
        id: "ROOT",
        name: "",
        type: "folder",
        children: props.nodes()
      }
    })
  );


  const selectedValue = () =>
    props.selectedId?.() ? [props.selectedId?.() as string] : undefined;

  return (
    <TreeView.Root
      class="file-tree-root"
      collection={collection()}
      aria-label={props.ariaLabel}
      expandedValue={props.expandedIds()}
      onExpandedChange={
        props.onExpandedChange
          ? ({ expandedValue }) => props.onExpandedChange?.(expandedValue)
          : undefined
      }
      selectedValue={selectedValue()}
      onSelectionChange={
        props.onSelect
          ? ({ selectedValue }) => {
              const next = selectedValue[0];
              if (next) props.onSelect?.(next);
            }
          : undefined
      }
    >
      <TreeView.Tree class="file-tree-tree">
        <VirtualTreeList
          class="file-tree"
          innerClass="file-tree-inner"
          windowClass="file-tree-window"
          items={flatIds}
          rowHeight={ROW_HEIGHT}
        >
          {(id) => {
            const entry = () => entryMap().get(id);
            const current = entry();
            if (!current) return null;
            return <FileTreeItem entry={current} />;
          }}
        </VirtualTreeList>
      </TreeView.Tree>
    </TreeView.Root>
  );
};
