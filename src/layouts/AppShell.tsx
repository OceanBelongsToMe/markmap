import type { JSX } from "solid-js";
import { createEffect } from "solid-js";
import { useWindowFullscreen } from "../state/useWindowFullscreen";

export type AppShellProps = {
  children: JSX.Element;
};

export const AppShell = (props: AppShellProps) => {
  const isFullscreen = useWindowFullscreen();

  createEffect(() => {
    document.documentElement.dataset.windowFullscreen = isFullscreen() ? "true" : "false";
  });

  return <div class="app-shell">{props.children}</div>;
};
