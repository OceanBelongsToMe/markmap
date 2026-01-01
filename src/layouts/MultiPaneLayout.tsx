import { createMemo } from "solid-js";
import type { JSX } from "solid-js";

export type Pane = {
  key: string;
  content: JSX.Element;
  width?: string;
};

export type MultiPaneLayoutProps = {
  panes: Pane[];
  class?: string;
};

export const MultiPaneLayout = (props: MultiPaneLayoutProps) => {
  const template = createMemo(() =>
    props.panes.map((pane) => pane.width ?? "1fr").join(" ")
  );

  return (
    <div
      class={props.class ? `multi-pane-layout ${props.class}` : "multi-pane-layout"}
      data-pane-count={props.panes.length}
      style={{
        display: "grid",
        "grid-template-columns": template(),
        width: "100%",
        height: "100%"
      }}
    >
      {props.panes.map((pane) => (
        <div>{pane.content}</div>
      ))}
    </div>
  );
};
