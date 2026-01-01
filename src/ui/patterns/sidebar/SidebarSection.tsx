import type { JSX } from "solid-js";

export type SidebarSectionProps = {
  title?: string;
  children: JSX.Element;
  collapsed?: boolean;
  onToggle?: () => void;
};

export const SidebarSection = (props: SidebarSectionProps) => {
  const className = [
    "sidebar-section",
    props.collapsed ? "is-collapsed" : ""
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <section class={className} data-collapsed={props.collapsed ? "true" : "false"}>
      {props.title ? (
        <div class="sidebar-section-header">
          <button
            class="sidebar-section-toggle"
            type="button"
            onClick={props.onToggle}
            aria-expanded={!props.collapsed}
          >
            <span class="sidebar-section-chevron" aria-hidden="true" />
            <span class="sidebar-section-title">{props.title}</span>
          </button>
        </div>
      ) : null}
      <div class="sidebar-section-body" aria-hidden={props.collapsed ? "true" : "false"}>
        {props.children}
      </div>
    </section>
  );
};
