import { JSX, Show, createMemo } from "solid-js";
import { SidebarShell } from "./SidebarShell";
import { Sash } from "./Sash";

export type SidebarMode = "fixed" | "overlay" | "hidden";

export type UnifiedSidebarShellProps = {
  children: JSX.Element;
  mode: SidebarMode;
  width: number;
  onWidthChange: (width: number) => void;
  onMouseEnter?: () => void;
  onMouseLeave?: () => void;
  class?: string;
  classList?: Record<string, boolean | undefined>;
};

export const UnifiedSidebarShell = (props: UnifiedSidebarShellProps) => {
  const isOverlay = createMemo(() => props.mode === "overlay");
  const isHidden = createMemo(() => props.mode === "hidden");

  return (
    <div
      class="unified-sidebar-container"
      style={{ width: `${props.width}px` }}
      classList={{
        "is-overlay": isOverlay(),
        "is-hidden": isHidden(),
        ...props.classList
      }}
      onMouseEnter={props.onMouseEnter}
      onMouseLeave={props.onMouseLeave}
    >
      <SidebarShell
        width={props.width}
        overlay={isOverlay()}
        hidden={isHidden()}
      >
        {props.children}
      </SidebarShell>

      <Show when={props.mode !== "hidden"}>
        <Sash
          left={props.width}
          onDrag={(x) => props.onWidthChange(x)}
        />
      </Show>
    </div>
  );
};
