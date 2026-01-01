import { createEffect, createSignal, onCleanup } from "solid-js";

export const useMeasuredWidth = (getElement: () => HTMLElement | undefined) => {
  const [width, setWidth] = createSignal(0);

  const update = () => {
    const el = getElement();
    if (!el) return;
    setWidth(el.getBoundingClientRect().width);
  };

  createEffect(() => {
    const el = getElement();
    if (!el) return;

    update();

    let observer: ResizeObserver | null = null;
    if (typeof ResizeObserver !== "undefined") {
      observer = new ResizeObserver(update);
      observer.observe(el);
    }

    const onResize = () => update();
    window.addEventListener("resize", onResize);

    onCleanup(() => {
      observer?.disconnect();
      window.removeEventListener("resize", onResize);
    });
  });

  return width;
};
