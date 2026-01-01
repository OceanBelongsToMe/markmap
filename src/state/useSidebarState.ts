import { createSignal } from "solid-js";

export const useSidebarState = () => {
  const [collapsed, setCollapsed] = createSignal(false);
  const [width, setWidth] = createSignal(240);

  return {
    collapsed,
    setCollapsed,
    width,
    setWidth
  };
};
