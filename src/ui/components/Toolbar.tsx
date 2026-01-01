import type { JSX } from "solid-js";

export type ToolbarProps = {
  children: JSX.Element;
  class?: string;
  "data-tauri-drag-region"?: boolean;
};

export const Toolbar = (props: ToolbarProps) => {
  const className = props.class ? `toolbar ${props.class}` : "toolbar";
  return (
    <div class={className} data-tauri-drag-region={props["data-tauri-drag-region"]}>
      {props.children}
    </div>
  );
};

export type ToolbarGroupProps = {
  children: JSX.Element;
  class?: string;
};

export const ToolbarGroup = (props: ToolbarGroupProps) => {
  const className = props.class ? `toolbar-group ${props.class}` : "toolbar-group";
  return <div class={className}>{props.children}</div>;
};

export const ToolbarSpacer = () => {
  return <div class="toolbar-spacer" />;
};
