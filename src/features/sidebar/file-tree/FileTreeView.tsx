import type { Accessor } from "solid-js";
import { For, createMemo, createSignal, onCleanup } from "solid-js";
import { createVirtualList } from "@solid-primitives/virtual";
import { ChevronIcon } from "../../../ui/components/ChevronIcon";
import { flattenFileTree, type FlatFileTreeNode } from "./flattenFileTree";
import type { FileTreeIcon, FileTreeNode } from "./types";
import "./fileTree.css";

export type FileTreeViewProps = {
  nodes: Accessor<FileTreeNode[]>;
  expandedIds: Accessor<string[]>;
  selectedId?: Accessor<string | null>;
  onNodeClick?: (node: FileTreeNode) => void;
  animateId?: Accessor<string | null>;
};

const ROW_HEIGHT = 28;

export const FileTreeView = (props: FileTreeViewProps) => {
  let resizeObserver: ResizeObserver | undefined;
  const [rootHeight, setRootHeight] = createSignal(0);
  const flatNodes = createMemo<FlatFileTreeNode[]>(() =>
    flattenFileTree(props.nodes(), props.expandedIds())
  );
  const flatIds = createMemo(() => flatNodes().map((node) => node.node.id));
  const entryMap = createMemo(() => {
    const map = new Map<string, FlatFileTreeNode>();
    for (const entry of flatNodes()) {
      map.set(entry.node.id, entry);
    }
    return map;
  });

  const virtualList = createMemo(() => {
    if (rootHeight() <= 0) return null;
    return createVirtualList({
      items: flatIds(),
      rootHeight: rootHeight(),
      rowHeight: ROW_HEIGHT,
      overscanCount: 6
    });
  });
  const virtualState = () => virtualList()?.[0]();
  const onScroll = (event: Event) => {
    virtualList()?.[1](event);
  };

  const setScrollRef = (el: HTMLDivElement) => {
    setRootHeight(el.clientHeight);
    resizeObserver?.disconnect();
    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        setRootHeight(entry.contentRect.height);
      }
    });
    resizeObserver.observe(el);
    onScroll({ target: el } as Event);
  };

  onCleanup(() => {
    resizeObserver?.disconnect();
  });

  return (
    <div
      class="file-tree"
      role="tree"
      ref={setScrollRef}
      onScroll={onScroll}
    >
      <div
        class="file-tree-inner"
        style={{ height: `${virtualState()?.containerHeight ?? 0}px` }}
      >
        <div
          class="file-tree-window"
          style={{ transform: `translateY(${virtualState()?.viewerTop ?? 0}px)` }}
        >
          <For each={virtualState()?.visibleItems ?? []} fallback={null}>
            {(id) => {
              const entry = () => entryMap().get(id);
            return (
                <div class="file-tree-item">
                  <FileTreeNodeRow
                    entry={entry}
                    selectedId={props.selectedId}
                    onNodeClick={props.onNodeClick}
                    animateId={props.animateId}
                  />
                </div>
              );
            }}
          </For>
        </div>
      </div>
    </div>
  );
};

type FileTreeNodeRowProps = {
  entry: Accessor<FlatFileTreeNode | undefined>;
  selectedId?: Accessor<string | null>;
  onNodeClick?: (node: FileTreeNode) => void;
  animateId?: Accessor<string | null>;
};

const FileTreeNodeRow = (props: FileTreeNodeRowProps) => {
  const entry = createMemo(() => props.entry());
  const isSelected = createMemo(() => {
    const current = entry();
    return current ? props.selectedId?.() === current.node.id : false;
  });

  const onClickRow = () => {
    const current = entry();
    if (!current) return;
    props.onNodeClick?.(current.node);
  };

  if (!entry()) return null;

  return (
    <div
      class="file-tree-node collapsible"
      data-type={entry()!.node.type}
      data-selected={isSelected()}
      data-collapsed={
        entry()!.isFolder ? (entry()!.isExpanded ? "false" : "true") : undefined
      }
      data-animate={props.animateId?.() === entry()!.node.id ? "true" : "false"}
      role="treeitem"
      aria-expanded={entry()!.isFolder ? entry()!.isExpanded : undefined}
      aria-selected={isSelected()}
    >
      <button
        type="button"
        class="file-tree-row collapsible-trigger"
        style={{
          "--depth": `${entry()!.depth}`,
          "--file-tree-row-height": `${ROW_HEIGHT}px`
        }}
        onClick={onClickRow}
        data-collapsed={
          entry()!.isFolder
            ? (entry()!.isExpanded ? "false" : "true")
            : undefined
        }
        data-animate={props.animateId?.() === entry()!.node.id ? "true" : "false"}
      >
        <span class="file-tree-caret collapsible-chevron" aria-hidden="true">
          <ChevronIcon class="collapsible-chevron-icon" />
        </span>
        <span class="file-tree-icon" aria-hidden="true">
          {renderIcon(entry()!.node)}
        </span>
        <span class="file-tree-label">{entry()!.node.name}</span>
      </button>
    </div>
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
