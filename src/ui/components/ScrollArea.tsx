import type { JSX } from "solid-js";
import "./scrollArea.css";

export type ScrollAreaProps = {
  children: JSX.Element;
  class?: string;
};

export const ScrollArea = (props: ScrollAreaProps) => {
  const className = props.class ? `scroll-area ${props.class}` : "scroll-area";
  return <div class={className}>{props.children}</div>;
};
