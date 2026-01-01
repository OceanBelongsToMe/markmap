import type { JSX } from "solid-js";

export type SidebarShellProps = {
  children: JSX.Element;
  collapsed?: boolean;
  width?: number;
  class?: string;
};

export const SidebarShell = (props: SidebarShellProps) => {
  const className = [
    "sidebar-shell",
    props.collapsed ? "is-collapsed" : "",
    props.class ?? ""
  ]
    .filter(Boolean)
    .join(" ");

  const style =
    props.width !== undefined ? { "--sidebar-width": `${props.width}px` } : undefined;

  return (
    <div class={className} style={style}>
      {props.children}
    </div>
  );
};
