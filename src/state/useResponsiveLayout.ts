import { createEffect, createMemo, createSignal, onCleanup } from "solid-js";
import {
  defaultLayoutMins,
  getLayoutVariant,
  type LayoutMinSizes
} from "../layouts/rules/layoutBreakpoints";
import { useMeasuredWidth } from "./useMeasuredWidth";

export const useResponsiveLayout = (
  getElement: () => HTMLElement | undefined,
  mins: LayoutMinSizes = defaultLayoutMins
) => {
  const width = useMeasuredWidth(getElement);
  const readViewportWidth = () => {
    if (typeof window === "undefined") return 0;
    return (
      window.innerWidth ||
      document.documentElement?.clientWidth ||
      document.body?.clientWidth ||
      0
    );
  };

  const [viewportWidth, setViewportWidth] = createSignal(readViewportWidth());

  createEffect(() => {
    const update = () => setViewportWidth(readViewportWidth());
    update();
    const onResize = () => update();
    window.addEventListener("resize", onResize);
    onCleanup(() => window.removeEventListener("resize", onResize));
  });

  const layoutVariant = createMemo(() => {
    const measured = width();
    const effectiveWidth = measured > 0 ? measured : viewportWidth();
    return getLayoutVariant(effectiveWidth, mins);
  });

  return {
    layoutVariant,
    width,
    viewportWidth
  };
};
