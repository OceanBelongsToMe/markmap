import type { JSX } from "solid-js";

export type SidebarShellProps = {
  children: JSX.Element;
  collapsed?: boolean;
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

  return <div class={className}>{props.children}</div>;
};
