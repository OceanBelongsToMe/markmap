import { createEffect, createSignal, onCleanup } from "solid-js";
import { captureFocus, restoreFocus, trapFocus } from "../../../a11y/focus";
import { useI18n } from "../../../i18n/context";

export const useFloatingEditorPanel = () => {
  const { t } = useI18n();
  const [isOpen, setIsOpen] = createSignal(false);
  let panelRef: HTMLDivElement | undefined;
  let previousFocus: HTMLElement | null = null;

  const setPanelRef = (el: HTMLDivElement) => {
    panelRef = el;
  };

  createEffect(() => {
    if (!isOpen() || !panelRef) return;

    previousFocus = captureFocus();
    const releaseTrap = trapFocus(panelRef, { onEscape: () => setIsOpen(false) });

    onCleanup(() => {
      releaseTrap();
      restoreFocus(previousFocus);
    });
  });

  return {
    t,
    isOpen,
    setIsOpen,
    setPanelRef
  };
};
