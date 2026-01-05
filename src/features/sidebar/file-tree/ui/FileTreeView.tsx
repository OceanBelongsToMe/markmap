import type { Accessor } from "solid-js";
import { createMemo } from "solid-js";
import { TreeView } from "@ark-ui/solid/tree-view";
import { flattenFileTree } from "../domain/flattenFileTree";
import type { FileTreeNode } from "../domain/types";
import { useFileTreeCollection } from "./useFileTreeCollection";
import { FileTreeVirtualList } from "./FileTreeVirtualList";
import type { FileTreeStyle } from "../style/fileTreeStyleTypes";
import "../styles/base.css";
import "../styles/interaction.css";
import "../styles/content.css";
import "../styles/theme-ark.css";
import "../styles/theme-dense.css";
import "../styles/theme-classic.css";

export type FileTreeViewProps = {
  nodes: Accessor<FileTreeNode[]>;
  expandedIds: Accessor<string[]>;
  selectedId?: Accessor<string | null>;
  ariaLabel?: string;
  onSelect?: (id: string) => void;
  onExpandedChange?: (ids: string[]) => void;
  style?: FileTreeStyle;
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

  const { collection, selectedValue, handleExpandedChange, handleSelectionChange } =
    useFileTreeCollection({
      nodes: props.nodes,
      selectedId: props.selectedId,
      onSelect: props.onSelect,
      onExpandedChange: props.onExpandedChange
    });

  return (
    <TreeView.Root
      class="file-tree-root"
      data-style={props.style ?? "ark"}
      collection={collection()}
      aria-label={props.ariaLabel}
      expandedValue={props.expandedIds()}
      onExpandedChange={handleExpandedChange}
      selectedValue={selectedValue()}
      onSelectionChange={handleSelectionChange}
    >
      <TreeView.Tree class="file-tree-tree">
        <FileTreeVirtualList
          flatIds={flatIds}
          entryMap={entryMap}
          rowHeight={ROW_HEIGHT}
          style={props.style}
        />
      </TreeView.Tree>
    </TreeView.Root>
  );
};
