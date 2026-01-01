import { createEffect, createSignal } from "solid-js";

export type PaneSize = {
  initialPx: number;
  minPx: number;
  maxPx?: number;
};

export type PaneConfig = {
  size?: PaneSize;
};

export const usePaneSizes = (getPanes: () => PaneConfig[], containerWidth: () => number) => {
  const [sizes, setSizes] = createSignal(
    getPanes().map((pane) => pane.size?.initialPx ?? 300)
  );

  const clampSizes = () => {
    const width = containerWidth();
    if (!width) return;
    const panes = getPanes();
    const current = sizes();
    const next = [...current];
    let remaining = width;
    const mins = panes.map((pane) => pane?.size?.minPx ?? 120);

    for (let i = 0; i < next.length; i += 1) {
      const min = panes[i]?.size?.minPx ?? 120;
      const max = panes[i]?.size?.maxPx ?? width;
      next[i] = Math.min(max, Math.max(min, next[i]));
      remaining -= next[i];
    }

    if (remaining >= 0 && next.length > 0) {
      const last = next.length - 1;
      next[last] = Math.max(panes[last]?.size?.minPx ?? 120, next[last] + remaining);
    } else if (remaining < 0) {
      let deficit = -remaining;
      for (let i = next.length - 1; i >= 0 && deficit > 0; i -= 1) {
        const min = mins[i] ?? 0;
        const reducible = Math.max(0, next[i] - min);
        const reduce = Math.min(deficit, reducible);
        next[i] -= reduce;
        deficit -= reduce;
      }

      if (deficit > 0) {
        const total = next.reduce((acc, value) => acc + value, 0);
        if (total > 0) {
          const ratio = width / total;
          for (let i = 0; i < next.length; i += 1) {
            next[i] = Math.max(32, Math.floor(next[i] * ratio));
          }

          if (next.length > 0) {
            const last = next.length - 1;
            const sum = next.slice(0, last).reduce((acc, value) => acc + value, 0);
            next[last] = Math.max(32, width - sum);
          }
        }
      }
    }

    if (next.some((value, index) => value !== current[index])) {
      setSizes(next);
    }
  };

  const handleDrag = (index: number, clientX: number, containerLeft: number, containerWidthValue: number) => {
    const panes = getPanes();
    const current = sizes();
    const start = current.slice(0, index).reduce((acc, w) => acc + w, 0);
    const total = current[index] + current[index + 1];
    const desired = clientX - containerLeft - start;

    const leftMin = panes[index]?.size?.minPx ?? 120;
    const leftMax = panes[index]?.size?.maxPx ?? containerWidthValue;
    const rightMin = panes[index + 1]?.size?.minPx ?? 120;
    const rightMax = panes[index + 1]?.size?.maxPx ?? containerWidthValue;

    let nextLeft = Math.min(leftMax, Math.max(leftMin, desired));
    let nextRight = total - nextLeft;

    if (nextRight < rightMin) {
      nextRight = rightMin;
      nextLeft = total - nextRight;
    } else if (nextRight > rightMax) {
      nextRight = rightMax;
      nextLeft = total - nextRight;
    }

    const next = [...current];
    next[index] = nextLeft;
    next[index + 1] = nextRight;
    setSizes(next);
  };

  createEffect(clampSizes);
  createEffect(() => {
    const panes = getPanes();
    if (sizes().length !== panes.length) {
      setSizes(panes.map((pane) => pane.size?.initialPx ?? 300));
    }
  });

  return {
    sizes,
    handleDrag
  };
};
