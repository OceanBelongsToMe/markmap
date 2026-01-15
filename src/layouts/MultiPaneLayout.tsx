import { For, createMemo } from "solid-js";
import type { JSX } from "solid-js";

export type Pane = {
  key: string;
  content: JSX.Element;
};

export type MultiPaneLayoutProps = {
  panes: Pane[];
  widths?: () => number[];
  class?: string;
};

export const MultiPaneLayout = (props: MultiPaneLayoutProps) => {
  const template = createMemo(() => {
    const widths = props.widths?.();
    if (widths && widths.length) {
      return widths.map((value) => `${value}px`).join(" ");
    }
    return props.panes.map(() => "1fr").join(" ");
  });

  return (
    <div
      class={props.class ? `multi-pane-layout ${props.class}` : "multi-pane-layout"}
      data-pane-count={props.panes.length}
      style={{
        display: "grid",
        "grid-template-columns": template()
      }}
    >
      <For each={props.panes}>
        {(pane) => (
          <div class="multi-pane-pane" data-pane-key={pane.key}>
            {pane.content}
          </div>
        )}
      </For>
    </div>
  );
};
