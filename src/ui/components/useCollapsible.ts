import type { Accessor } from "solid-js";

export type CollapsibleController = {
  isOpen: Accessor<boolean>;
  isCollapsed: Accessor<boolean>;
  toggle: () => void;
};

export const useCollapsible = (open: Accessor<boolean>, onToggle?: () => void) => {
  const isOpen = () => open();
  const isCollapsed = () => !open();
  const toggle = () => onToggle?.();

  return {
    isOpen,
    isCollapsed,
    toggle
  } satisfies CollapsibleController;
};
