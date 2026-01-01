import { FloatingEditorPanelView } from "./components/FloatingEditorPanelView";
import { useFloatingEditorPanel } from "./hooks/useFloatingEditorPanel";

export const FloatingEditorPanelContent = () => {
  const { t, isOpen, setIsOpen, setPanelRef } = useFloatingEditorPanel();

  return (
    <FloatingEditorPanelView
      title={t("floatingEditorPanelTitle")}
      placeholder={t("floatingEditorPanelPlaceholder")}
      closeLabel={t("close")}
      isOpen={isOpen}
      onClose={() => setIsOpen(false)}
      onRef={setPanelRef}
    />
  );
};
