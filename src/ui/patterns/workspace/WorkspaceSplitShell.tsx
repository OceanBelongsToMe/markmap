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
  const sidebarSize = createMemo<PaneSize>(() => ({
    ...workspacePaneSizes.sidebar,
    initialPx: untrack(() => props.sidebarWidth) ?? workspacePaneSizes.sidebar.initialPx
  }));

  const sidebarPane: Pane & { size?: PaneSize; key: string } = {
    key: "sidebar",
    get content() {
      return props.sidebar as JSX.Element;
    },
    get size() {
      return sidebarSize();
    }
  };

  const editorPane: Pane & { size?: PaneSize; key: string } = {
    key: "editor",
    get content() {
      return props.editor;
    },
    size: workspacePaneSizes.editor
  };

  const previewPane: Pane & { size?: PaneSize; key: string } = {
    key: "preview",
    get content() {
      return props.preview as JSX.Element;
    },
    size: workspacePaneSizes.preview
  };

  const panes = createMemo<Array<Pane & { size?: PaneSize; key: string }>>(() => {
    const next: Array<Pane & { size?: PaneSize; key: string }> = [];

    if (props.sidebar) {
      next.push(sidebarPane);
    }

    next.push(editorPane);

    if (props.preview) {
      next.push(previewPane);
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
