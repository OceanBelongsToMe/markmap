import { createSignal, onCleanup } from "solid-js";

export const useTogglePulse = (durationMs = 220) => {
  const [lastToggledId, setLastToggledId] = createSignal<string | null>(null);
  let clearTimer: number | undefined;

  const markToggled = (id: string | null) => {
    if (!id) return;
    setLastToggledId(id);
    if (clearTimer) {
      window.clearTimeout(clearTimer);
    }
    clearTimer = window.setTimeout(() => setLastToggledId(null), durationMs);
  };

  onCleanup(() => {
    if (clearTimer) {
      window.clearTimeout(clearTimer);
    }
  });

  return {
    lastToggledId,
    markToggled
  };
};
