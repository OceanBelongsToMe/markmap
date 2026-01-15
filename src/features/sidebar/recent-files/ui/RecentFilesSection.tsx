import { For, Show, createMemo, createSignal, getOwner, runWithOwner } from "solid-js";
import { SidebarSection } from "../../../../ui/patterns/sidebar/SidebarSection";
import { useI18n } from "../../../../i18n/context";
import { RecentFileList } from "./RecentFileList";
import { useActiveDocument } from "../../../../state/workspace/useActiveDocument";
import { useWorkspaceTreeState } from "../../../../state/workspace/useWorkspaceTree";
import { useWorkspaceRecentFiles } from "../data/useWorkspaceRecentFiles";
import { mapRecentFilesToEntries } from "../data/mapRecentFilesToEntries";
import { groupRecentFilesByTime } from "../data/groupRecentFilesByTime";
import type { FileTreeStyle } from "../../file-tree/style/fileTreeStyleTypes";
import "../styles/recent-files.css";

export type RecentFilesSectionProps = {
  ariaLabel?: string;
  style?: FileTreeStyle;
  class?: string;
};

export const RecentFilesSection = (props: RecentFilesSectionProps) => {
  const { t } = useI18n();
  const { activeDocId, openDocument } = useActiveDocument();
  const { fileTree } = useWorkspaceTreeState();
  const recent = useWorkspaceRecentFiles();
  const [expandedIds, setExpandedIds] = createSignal<string[]>([]);
  const [collapsed, setCollapsed] = createSignal(false);

  const entries = createMemo(() =>
    mapRecentFilesToEntries(recent.items(), fileTree())
  );
  const groups = createMemo(() => groupRecentFilesByTime(entries()));
  const hasItems = () => entries().length > 0;
  const handleSelect = (id: string) => openDocument(id);
  const owner = getOwner();
  const handleReachEnd = () => {
    if (owner) {
      runWithOwner(owner, () => recent.loadMore());
    } else {
      recent.loadMore();
    }
  };

  return (
    <Show when={hasItems()}>
      <SidebarSection
        title={t("recent")}
        collapsed={collapsed()}
        onToggle={() => setCollapsed(!collapsed())}
        class={props.class}
      >
        <For each={groups()}>
          {(group, index) => {
            const nodes = () => group.entries.map((entry) => entry.node);
            const isLast = () => index() === groups().length - 1;
            return (
              <div class="recent-files-group">
                <div class="recent-files-group-title">{t(group.labelKey)}</div>
                <RecentFileList
                  entries={group.entries}
                  selectedId={activeDocId}
                  ariaLabel={props.ariaLabel}
                  onSelect={handleSelect}
                  onReachEnd={isLast() ? handleReachEnd : undefined}
                  style={props.style}
                />
              </div>
            );
          }}
        </For>
      </SidebarSection>
    </Show>
  );
};
