import { createSignal } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { WorkspacePanel } from "../../../features/workspace/WorkspacePanel";
import type { FileTreeStyle } from "../../../features/sidebar/file-tree/style/fileTreeStyleTypes";

export type WorkspaceEditorPaneProps = {
  fileTreeStyle: FileTreeStyle;
  onFileTreeStyleChange: (style: FileTreeStyle) => void;
};

export const WorkspaceEditorPane = (props: WorkspaceEditorPaneProps) => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());

  return (
    <EditorPane ref={(el) => setEditorRef(el)} class={`content-${contentVariant()}`}>
      <div style={{ padding: "12px 0" }}>
        <label style={{ display: "inline-flex", "align-items": "center", gap: "8px" }}>
          <span>File tree style</span>
          <select
            value={props.fileTreeStyle}
            onChange={(event) =>
              props.onFileTreeStyleChange(event.currentTarget.value as FileTreeStyle)
            }
          >
            <option value="ark">ark</option>
            <option value="dense">dense</option>
            <option value="classic">classic</option>
          </select>
        </label>
      </div>
      <WorkspacePanel />
    </EditorPane>
  );
};
