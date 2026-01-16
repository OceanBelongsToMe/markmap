import { createMemo } from "solid-js";
import type { JSX } from "solid-js";
import type { Pane } from "../../../layouts/MultiPaneLayout";
import { workspacePaneSizes } from "../../../layouts/rules/workspaceLayoutSizes";
import type { PaneSize } from "../../../state/usePaneSizes";
import { MultiPaneShell } from "../MultiPaneShell";

export type WorkspaceSplitShellProps = {
  editor: JSX.Element;
  preview?: JSX.Element;
  onSizesChange?: (sizes: number[]) => void;
};

export const WorkspaceSplitShell = (props: WorkspaceSplitShellProps) => {
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
