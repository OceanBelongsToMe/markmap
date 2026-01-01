import type { JSX } from "solid-js";
import { FloatingEditorPanel } from "../../../layouts/Regions";

export type WorkspaceFloatingPanelProps = {
  children: JSX.Element;
};

export const WorkspaceFloatingPanel = (props: WorkspaceFloatingPanelProps) => {
  return <FloatingEditorPanel>{props.children}</FloatingEditorPanel>;
};
