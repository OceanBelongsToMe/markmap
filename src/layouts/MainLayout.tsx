import type { JSX } from "solid-js";
import { AppShell } from "./AppShell";
import { ContentStage, FloatingPanelLayer, SidebarLayer, ToolbarRegion } from "./Layers";

export type MainLayoutProps = {
  sidebar?: JSX.Element;
  toolbar?: JSX.Element;
  content: JSX.Element;
  floatingPanel?: JSX.Element;
};

export const MainLayout = (props: MainLayoutProps) => {
  return (
    <AppShell>
      {props.toolbar ? <ToolbarRegion>{props.toolbar}</ToolbarRegion> : null}
      {props.sidebar ? <SidebarLayer>{props.sidebar}</SidebarLayer> : null}
      <ContentStage>{props.content}</ContentStage>
      {props.floatingPanel ? (
        <FloatingPanelLayer>{props.floatingPanel}</FloatingPanelLayer>
      ) : null}
    </AppShell>
  );
};
