import { createSignal } from "solid-js";

export type LayoutMode = "single" | "split";

export const useLayoutState = () => {
  const [layoutMode, setLayoutMode] = createSignal<LayoutMode>("split");

  return {
    layoutMode,
    setLayoutMode
  };
};
