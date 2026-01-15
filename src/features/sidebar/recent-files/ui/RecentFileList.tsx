import type { Accessor } from "solid-js";
import { For } from "solid-js";
import type { FileTreeStyle } from "../../file-tree/style/fileTreeStyleTypes";
import { FileTreeRow } from "../../file-tree/ui/FileTreeRow";
import type { RecentFileEntry } from "../data/mapRecentFilesToEntries";
import "../../file-tree/styles/base.css";
import "../../file-tree/styles/interaction.css";
import "../../file-tree/styles/content.css";
import "../../file-tree/styles/theme-ark.css";
import "../../file-tree/styles/theme-dense.css";
import "../../file-tree/styles/theme-classic.css";
import "../styles/recent-files.css";

export type RecentFileListProps = {
  entries: RecentFileEntry[];
  selectedId?: Accessor<string | null>;
  ariaLabel?: string;
  onSelect?: (id: string) => void;
  onReachEnd?: () => void;
  style?: FileTreeStyle;
};

export const RecentFileList = (props: RecentFileListProps) => {
  const reachThreshold = 2;
  let listEl: HTMLDivElement | undefined;

  const handleScroll = () => {
    if (!listEl || !props.onReachEnd) return;
    const rowHeight = 28;
    const threshold = rowHeight * reachThreshold;
    if (listEl.scrollTop + listEl.clientHeight >= listEl.scrollHeight - threshold) {
      props.onReachEnd();
    }
  };

  const handleSelect = (id: string) => props.onSelect?.(id);
  return (
    <div
      class="recent-file-list file-tree file-tree-root"
      data-style={props.style ?? "ark"}
      ref={(el) => {
        listEl = el;
      }}
      onScroll={handleScroll}
    >
      <For each={props.entries}>
        {(entry) => {
          const isSelected = () =>
            props.selectedId?.() ? props.selectedId?.() === entry.documentId : false;
          return (
            <button
              class="recent-file-item"
              type="button"
              data-part="item"
              data-selected={isSelected() ? "true" : "false"}
              aria-selected={isSelected() ? "true" : "false"}
              style={{ "--depth": "0" }}
              onClick={() => handleSelect(entry.documentId)}
            >
              <FileTreeRow node={entry.node} style={props.style} />
            </button>
          );
        }}
      </For>
    </div>
  );
};
