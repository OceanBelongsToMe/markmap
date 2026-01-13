import { Show } from "solid-js";
import { FloatingEditorPanelView } from "./components/FloatingEditorPanelView";
import { useFloatingEditorPanel } from "./hooks/useFloatingEditorPanel";
import { WorkspaceFloatingPanel } from "../../ui/patterns/workspace/WorkspaceFloatingPanel";

export const FloatingEditorPanelContent = () => {
  const { t, isOpen, setIsOpen, setPanelRef } = useFloatingEditorPanel();

  return (
    <Show when={isOpen()}>
      <WorkspaceFloatingPanel>
        <FloatingEditorPanelView
          title={t("floatingEditorPanelTitle")}
          placeholder={t("floatingEditorPanelPlaceholder")}
          closeLabel={t("close")}
          isOpen={isOpen}
          onClose={() => setIsOpen(false)}
          onRef={setPanelRef}
        />
      </WorkspaceFloatingPanel>
    </Show>
  );
};
