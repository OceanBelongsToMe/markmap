import type { JSX } from "solid-js";

export type LayerProps = {
  children: JSX.Element;
};

export const SidebarLayer = (props: LayerProps) => {
  return <aside class="sidebar-layer">{props.children}</aside>;
};

export const ContentStage = (props: LayerProps) => {
  return <div class="content-stage">{props.children}</div>;
};

export const ToolbarRegion = (props: LayerProps) => {
  return (
    <div class="toolbar-region" data-tauri-drag-region>
      {props.children}
    </div>
  );
};

export const FloatingPanelLayer = (props: LayerProps) => {
  return <div class="floating-panel-layer">{props.children}</div>;
};
