import type { Accessor } from "solid-js";
import { VirtualTreeList } from "../../../../ui/virtual/VirtualTreeList";
import { FileTreeItem } from "./FileTreeItem";
import type { FlatFileTreeNode } from "../domain/flattenFileTree";
import type { FileTreeStyle } from "../style/fileTreeStyleTypes";

export type FileTreeVirtualListProps = {
  flatIds: Accessor<string[]>;
  entryMap: Accessor<Map<string, FlatFileTreeNode>>;
  rowHeight: number;
  style?: FileTreeStyle;
};

export const FileTreeVirtualList = (props: FileTreeVirtualListProps) => {
  return (
    <VirtualTreeList
      class="file-tree"
      innerClass="file-tree-inner"
      windowClass="file-tree-window"
      items={props.flatIds}
      rowHeight={props.rowHeight}
    >
      {(id) => {
        const entry = () => props.entryMap().get(id);
        const current = entry();
        if (!current) return null;
        return <FileTreeItem entry={current} style={props.style} />;
      }}
    </VirtualTreeList>
  );
};
