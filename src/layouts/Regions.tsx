import type { JSX } from "solid-js";

export type RegionProps = {
  children?: JSX.Element;
  class?: string;
  ref?: (el: HTMLDivElement) => void;
};

export const Sidebar = (props: RegionProps) => {
  const className = props.class ? `sidebar-region ${props.class}` : "sidebar-region";
  return (
    <div class={className} ref={props.ref}>
      {props.children}
    </div>
  );
};

export const EditorPane = (props: RegionProps) => {
  const className = props.class ? `editor-pane ${props.class}` : "editor-pane";
  return (
    <div class={className} ref={props.ref}>
      {props.children}
    </div>
  );
};

export const MarkmapStage = (props: RegionProps) => {
  const className = props.class ? `markmap-stage ${props.class}` : "markmap-stage";
  return (
    <div class={className} ref={props.ref}>
      {props.children}
    </div>
  );
};

export const PreviewPane = (props: RegionProps) => {
  const className = props.class ? `preview-pane ${props.class}` : "preview-pane";
  return (
    <div class={className} ref={props.ref}>
      {props.children}
    </div>
  );
};

export const FloatingEditorPanel = (props: RegionProps) => {
  const className = props.class
    ? `floating-editor-panel ${props.class}`
    : "floating-editor-panel";
  return (
    <div class={className} ref={props.ref}>
      {props.children}
    </div>
  );
};
