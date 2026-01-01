import type { JSX } from "solid-js";
import { Toolbar, ToolbarGroup, ToolbarSpacer } from "../components/Toolbar";

export type ToolbarShellProps = {
  left?: JSX.Element;
  center?: JSX.Element;
  right?: JSX.Element;
  class?: string;
};

export const ToolbarShell = (props: ToolbarShellProps) => {
  return (
    <Toolbar class={props.class} data-tauri-drag-region>
      <div class="toolbar-shell-content">
        {props.left ? <ToolbarGroup>{props.left}</ToolbarGroup> : null}
        {props.center ? (
          <>
            <ToolbarSpacer />
            <ToolbarGroup>{props.center}</ToolbarGroup>
          </>
        ) : null}
        {props.right ? (
          <>
            <ToolbarSpacer />
            <ToolbarGroup>{props.right}</ToolbarGroup>
          </>
        ) : null}
      </div>
    </Toolbar>
  );
};
