import type { JSX } from "solid-js";

export type AppShellProps = {
  children: JSX.Element;
};

export const AppShell = (props: AppShellProps) => {
  return <div class="app-shell">{props.children}</div>;
};
