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
    let resizeTimer: number | undefined;
    const computeSizeFullscreen = () => {
      const { width, height, availWidth, availHeight } = window.screen;
      const matchesScreen =
        Math.abs(window.innerWidth - width) <= 1 &&
        Math.abs(window.innerHeight - height) <= 1;
      const matchesAvail =
        Math.abs(window.innerWidth - availWidth) <= 1 &&
        Math.abs(window.innerHeight - availHeight) <= 1;
      return matchesScreen || matchesAvail;
    };
    const scheduleSizeCheck = () => {
      if (resizeTimer) {
        window.clearTimeout(resizeTimer);
      }
      resizeTimer = window.setTimeout(() => {
        setIsFullscreen(computeSizeFullscreen());
      }, 200);
    };
    const syncFullscreenState = async () => {
      try {
        const windowFullscreen = await appWindow.isFullscreen();
        if (windowFullscreen) {
          setIsFullscreen(true);
          return;
        }
        scheduleSizeCheck();
      } catch {
        scheduleSizeCheck();
      }
    };

    void syncFullscreenState();
    let unlistenAppResize: undefined | (() => void);

    const handleResize = () => {
      void syncFullscreenState();
    };
    window.addEventListener("resize", handleResize);

    appWindow
      .onResized(() => {
        void syncFullscreenState();
      })
      .then((stop) => {
        unlistenAppResize = stop;
      })
      .catch(() => {
        unlistenAppResize = undefined;
      });

    onCleanup(() => {
      window.removeEventListener("resize", handleResize);
      if (resizeTimer) {
        window.clearTimeout(resizeTimer);
      }
      if (unlistenAppResize) {
        unlistenAppResize();
      }
    });
  });

  return isFullscreen;
};
