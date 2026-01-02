import type { JSX } from "solid-js";
import { StableList } from "../../../ui/components/StableList";
import type { FileTreeNode } from "./useFileTreeData";

export type FileTreeViewProps = {
  nodes: FileTreeNode[];
  selectedId?: string | null;
  onSelect?: (id: string) => void;
  renderLabel?: (node: FileTreeNode) => JSX.Element;
};

export const FileTreeView = (props: FileTreeViewProps) => {
  return (
    <div>
      <StableList each={() => props.nodes}>
        {(node) => (
          <div>
            <button type="button" onClick={() => props.onSelect?.(node().id)}>
              {props.renderLabel ? props.renderLabel(node()) : node().label}
            </button>
          </div>
        )}
      </StableList>
    </div>
  );
};
