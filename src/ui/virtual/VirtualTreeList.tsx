import type { Accessor, JSX } from "solid-js";
import { For, createEffect, createMemo, createSignal, onCleanup } from "solid-js";
import { createVirtualList } from "@solid-primitives/virtual";

export type VirtualTreeListProps<T> = {
  items: Accessor<T[]>;
  rowHeight: number;
  overscanCount?: number;
  class?: string;
  innerClass?: string;
  windowClass?: string;
  children: (item: T) => JSX.Element;
};

export const VirtualTreeList = <T,>(props: VirtualTreeListProps<T>) => {
  let resizeObserver: ResizeObserver | undefined;
  const [rootHeight, setRootHeight] = createSignal(0);
  const [scrollEl, setScrollEl] = createSignal<HTMLDivElement | null>(null);

  const virtualList = createMemo(() => {
    const items = props.items();
    if (rootHeight() <= 0) return null;
    return createVirtualList({
      items,
      rootHeight: rootHeight(),
      rowHeight: props.rowHeight,
      overscanCount: props.overscanCount ?? 6
    });
  });

  const virtualState = () => virtualList()?.[0]();
  const visibleItems = () => virtualState()?.visibleItems ?? props.items();
  const containerHeight = () =>
    virtualState()?.containerHeight ?? props.items().length * props.rowHeight;
  const viewerTop = () => virtualState()?.viewerTop ?? 0;
  const onScroll = (event: Event) => {
    virtualList()?.[1](event);
  };

  const measureHeight = (el: HTMLDivElement) => {
    const direct = el.clientHeight;
    if (direct > 0) {
      setRootHeight(direct);
      return;
    }
    const parent = el.parentElement?.clientHeight ?? 0;
    if (parent > 0) {
      setRootHeight(parent);
    }
  };

  const setScrollRef = (el: HTMLDivElement) => {
    setScrollEl(el);
    measureHeight(el);
    requestAnimationFrame(() => measureHeight(el));
    resizeObserver?.disconnect();
    resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const height = entry.contentRect.height;
        if (height > 0) {
          setRootHeight(height);
        }
      }
    });
    resizeObserver.observe(el);
    onScroll({ target: el } as unknown as Event);
  };

  createEffect(() => {
    props.items();
    const el = scrollEl();
    if (el) {
      onScroll({ target: el } as unknown as Event);
    }
  });

  onCleanup(() => {
    resizeObserver?.disconnect();
  });

  return (
    <div class={props.class} ref={setScrollRef} onScroll={onScroll}>
      <div class={props.innerClass ?? ""} style={{ height: `${containerHeight()}px` }}>
        <div
          class={props.windowClass ?? ""}
          style={{ transform: `translateY(${viewerTop()}px)` }}
        >
          <For each={visibleItems()} fallback={null}>
            {(item) => props.children(item)}
          </For>
        </div>
      </div>
    </div>
  );
};
