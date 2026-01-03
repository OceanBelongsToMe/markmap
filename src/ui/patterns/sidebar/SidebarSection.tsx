import type { JSX } from "solid-js";
import { ChevronRight } from "lucide-solid";
import { useCollapsible } from "../../components/useCollapsible";

export type SidebarSectionProps = {
  title?: string;
  children: JSX.Element;
  collapsed?: boolean;
  onToggle?: () => void;
  class?: string;
};

export const SidebarSection = (props: SidebarSectionProps) => {
  const { isOpen, isCollapsed, toggle } = useCollapsible(
    () => !props.collapsed,
    props.onToggle
  );
  const className = [
    "sidebar-section",
    "collapsible",
    props.class ?? "",
    isCollapsed() ? "is-collapsed" : ""
  ]
    .filter(Boolean)
    .join(" ");

  return (
    <section class={className} data-collapsed={isCollapsed() ? "true" : "false"}>
      {props.title ? (
        <div class="sidebar-section-header">
          <button
            class="sidebar-section-toggle collapsible-trigger"
            type="button"
            onClick={toggle}
            aria-expanded={isOpen()}
            data-collapsed={isCollapsed() ? "true" : "false"}
          >
            <span
              class="sidebar-section-chevron collapsible-chevron"
              aria-hidden="true"
            >
              <ChevronRight class="collapsible-chevron-icon" />
            </span>
            <span class="sidebar-section-title">{props.title}</span>
          </button>
        </div>
      ) : null}
      <div
        class="sidebar-section-body collapsible-body"
        aria-hidden={isCollapsed() ? "true" : "false"}
      >
        {props.children}
      </div>
    </section>
  );
};
