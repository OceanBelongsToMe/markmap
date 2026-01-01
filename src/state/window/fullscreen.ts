import { isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { createSignal, onCleanup, onMount } from "solid-js";

export const useWindowFullscreen = () => {
  const [isFullscreen, setIsFullscreen] = createSignal(false);

  onMount(() => {
    if (!isTauri()) {
      return;
    }

    const appWindow = getCurrentWindow();
    const syncFullscreenState = async () => {
      try {
        const windowFullscreen = await appWindow.isFullscreen();
        setIsFullscreen(windowFullscreen);
      } catch {
        setIsFullscreen(false);
      }
    };

    void syncFullscreenState();
    let unlistenFullscreenEvent: undefined | (() => void);

    appWindow
      .listen<boolean>("window-fullscreen-changed", (event) => {
        setIsFullscreen(Boolean(event.payload));
      })
      .then((stop) => {
        unlistenFullscreenEvent = stop;
      })
      .catch(() => {
        unlistenFullscreenEvent = undefined;
      });

    onCleanup(() => {
      if (unlistenFullscreenEvent) {
        unlistenFullscreenEvent();
      }
    });
  });

  return isFullscreen;
};
