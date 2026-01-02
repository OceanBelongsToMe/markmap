import { createMemo, untrack } from "solid-js";
import type { JSX } from "solid-js";
import type { Pane } from "../../../layouts/MultiPaneLayout";
import { workspacePaneSizes } from "../../../layouts/rules/workspaceLayoutSizes";
import type { PaneSize } from "../../../state/usePaneSizes";
import { MultiPaneShell } from "../MultiPaneShell";

export type WorkspaceSplitShellProps = {
  sidebar?: JSX.Element;
  editor: JSX.Element;
  preview?: JSX.Element;
  sidebarWidth?: number;
  onSizesChange?: (sizes: number[]) => void;
};

export const WorkspaceSplitShell = (props: WorkspaceSplitShellProps) => {
  const panes = createMemo<Array<Pane & { size?: PaneSize }>>(() => {
    const next: Array<Pane & { size?: PaneSize }> = [];

    if (props.sidebar) {
      next.push({
        key: "sidebar",
        content: props.sidebar,
        size: {
          ...workspacePaneSizes.sidebar,
          initialPx:
            untrack(() => props.sidebarWidth) ?? workspacePaneSizes.sidebar.initialPx
        }
      });
    }

    next.push({
      key: "editor",
      content: props.editor,
      size: workspacePaneSizes.editor
    });

    if (props.preview) {
      next.push({
        key: "preview",
        content: props.preview,
        size: workspacePaneSizes.preview
      });
    }

    return next;
  });

  return (
    <MultiPaneShell
      class="workspace-shell"
      panes={panes}
      onSizesChange={props.onSizesChange}
    />
  );
};
