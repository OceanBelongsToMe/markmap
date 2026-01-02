import { createMemo, createSignal } from "solid-js";
import { MultiPaneLayout, type Pane } from "../../layouts/MultiPaneLayout";
import { useMeasuredWidth } from "../../state/useMeasuredWidth";
import { usePaneSizes, type PaneSize } from "../../state/usePaneSizes";
import { SashContainer } from "./SashContainer";

export type MultiPaneShellProps = {
  panes: () => Array<Pane & { size?: PaneSize }>;
  class?: string;
  onSizesChange?: (sizes: number[]) => void;
};

export const MultiPaneShell = (props: MultiPaneShellProps) => {
  const [containerRef, setContainerRef] = createSignal<HTMLDivElement | undefined>();
  const panes = createMemo(() => props.panes());
  const containerWidth = useMeasuredWidth(containerRef);
  const offsetLeft = createMemo(() => {
    containerWidth();
    return containerRef()?.getBoundingClientRect().left ?? 0;
  });
  const { sizes, handleDrag } = usePaneSizes(() => panes(), containerWidth);
  return (
    <div
      class={props.class}
      ref={(el) => setContainerRef(el)}
      style={{ position: "relative", width: "100%", height: "100%" }}
    >
      <MultiPaneLayout panes={panes()} widths={sizes} />
      <SashContainer
        class="sash-container full-window"
        sizes={sizes()}
        offsetLeft={offsetLeft()}
        onDrag={(index, x) => {
          const el = containerRef();
          if (!el) return;
          const rect = el.getBoundingClientRect();
          handleDrag(index, x, rect.left, rect.width);
        }}
        onDragEnd={(nextSizes) => props.onSizesChange?.(nextSizes)}
      />
    </div>
  );
};
