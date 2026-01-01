import { createSignal } from "solid-js";
import { EditorPane } from "../../../layouts/Regions";
import { useResponsiveContent } from "../../../state/useResponsiveContent";
import { WorkspacePanel } from "../../../features/workspace/WorkspacePanel";

export const WorkspaceEditorPane = () => {
  const [editorRef, setEditorRef] = createSignal<HTMLDivElement | undefined>();
  const { contentVariant } = useResponsiveContent(() => editorRef());

  return (
    <EditorPane ref={(el) => setEditorRef(el)} class={`content-${contentVariant()}`}>
      <WorkspacePanel />
    </EditorPane>
  );
};
