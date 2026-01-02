import type { JSX } from "solid-js";
import { useCollapsible } from "../../components/useCollapsible";

export type SidebarSectionProps = {
  title?: string;
  children: JSX.Element;
  collapsed?: boolean;
  onToggle?: () => void;
};

export const SidebarSection = (props: SidebarSectionProps) => {
  const { isOpen, isCollapsed, toggle } = useCollapsible(
    () => !props.collapsed,
    props.onToggle
  );
  const className = [
    "sidebar-section",
    isCollapsed() ? "is-collapsed" : ""
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <section class={className} data-collapsed={isCollapsed() ? "true" : "false"}>
      {props.title ? (
        <div class="sidebar-section-header">
          <button
            class="sidebar-section-toggle"
            type="button"
            onClick={toggle}
            aria-expanded={isOpen()}
          >
            <span class="sidebar-section-chevron" aria-hidden="true" />
            <span class="sidebar-section-title">{props.title}</span>
          </button>
        </div>
      ) : null}
      <div class="sidebar-section-body" aria-hidden={isCollapsed() ? "true" : "false"}>
        {props.children}
      </div>
    </section>
  );
};
