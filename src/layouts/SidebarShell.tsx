import type { JSX } from "solid-js";

export type SidebarShellProps = {
  children: JSX.Element;
  collapsed?: boolean;
  overlay?: boolean;
  hidden?: boolean;
  width?: number;
  class?: string;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
};

export const SidebarShell = (props: SidebarShellProps) => {
  const className = [
    "sidebar-shell",
    props.collapsed ? "is-collapsed" : "",
    props.overlay ? "is-overlay" : "",
    props.hidden ? "is-hidden" : "",
    props.class ?? ""
  ]
    .filter(Boolean)
    .join(" ");

  const style =
    props.width !== undefined ? { "--sidebar-width": `${props.width}px` } : undefined;

  return (
    <div
      class={className}
      style={style}
      onMouseEnter={props.onMouseEnter}
      onMouseLeave={props.onMouseLeave}
    >
      {props.children}
    </div>
  );
};