import type { JSX } from "solid-js";

export type SidebarShellProps = {
  children: JSX.Element;
  collapsed?: boolean;
  overlay?: boolean; // New prop
  hidden?: boolean; // New prop for animation
  width?: number;
  class?: string;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
};

export const SidebarShell = (props: SidebarShellProps) => {
  const style =
    props.width !== undefined ? { "--sidebar-width": `${props.width}px` } : undefined;

  return (
    <div
      class="sidebar-shell"
      classList={{
        "is-collapsed": props.collapsed,
        "is-overlay": props.overlay,
        "is-hidden": props.hidden,
        [props.class ?? ""]: !!props.class
      }}
      style={style}
      onMouseEnter={props.onMouseEnter}
      onMouseLeave={props.onMouseLeave}
    >
      {props.children}
    </div>
  );
};
