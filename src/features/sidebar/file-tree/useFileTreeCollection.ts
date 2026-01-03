import { createMemo } from "solid-js";
import type { Accessor } from "solid-js";
import { createTreeCollection } from "@ark-ui/solid/tree-view";
import type { FileTreeNode } from "./types";

type UseFileTreeCollectionArgs = {
  nodes: Accessor<FileTreeNode[]>;
  selectedId?: Accessor<string | null>;
  onSelect?: (id: string) => void;
  onExpandedChange?: (ids: string[]) => void;
};

export const useFileTreeCollection = (args: UseFileTreeCollectionArgs) => {
  const collection = createMemo(() =>
    createTreeCollection<FileTreeNode>({
      nodeToValue: (node) => node.id,
      nodeToString: (node) => node.name,
      rootNode: {
        id: "ROOT",
        name: "",
        type: "folder",
        children: args.nodes()
      }
    })
  );

  const selectedValue = () =>
    args.selectedId?.() ? [args.selectedId?.() as string] : undefined;

  const handleExpandedChange = args.onExpandedChange
    ? ({ expandedValue }: { expandedValue: string[] }) =>
        args.onExpandedChange?.(expandedValue)
    : undefined;

  const handleSelectionChange = args.onSelect
    ? ({ selectedValue }: { selectedValue: string[] }) => {
        const next = selectedValue[0];
        if (next) args.onSelect?.(next);
      }
    : undefined;

  return {
    collection,
    selectedValue,
    handleExpandedChange,
    handleSelectionChange
  };
};
