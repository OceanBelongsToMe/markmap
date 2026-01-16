import { createEffect, createSignal, createMemo, on } from "solid-js";

export type PaneSize = {
  initialPx: number;
  minPx: number;
  maxPx?: number;
};

export type PaneConfig = {
  key: string; // Must have key for persistence
  size?: PaneSize;
};

export const usePaneSizes = (getPanes: () => PaneConfig[], containerWidth: () => number) => {
  // 1. Persistence State: Map<Key, Width>
  const [sizeMap, setSizeMap] = createSignal<Record<string, number>>({});

  // 2. Computed Output: number[] based on current panes
  // This satisfies DIP: The View depends on this array, but the State is decoupled.
  const sizes = createMemo(() => {
    const map = sizeMap();
    return getPanes().map((pane) => {
      // If key exists in map, use it; otherwise use initialPx
      return map[pane.key] ?? pane.size?.initialPx ?? 300;
    });
  });

  // 3. Logic: Layout Clamping (Adapted for Map)
  const clampSizes = () => {
    const width = containerWidth();
    if (!width) return;
    const panes = getPanes();
    if (panes.length === 0) return;

    // Extract current widths
    const currentSizes = sizes();
    const nextSizes = [...currentSizes];
    
    // ... (Keep existing clamp logic, operating on the array) ...
    let remaining = width;
    const mins = panes.map((pane) => pane?.size?.minPx ?? 120);

    for (let i = 0; i < nextSizes.length; i += 1) {
      const min = panes[i]?.size?.minPx ?? 120;
      const max = panes[i]?.size?.maxPx ?? width;
      nextSizes[i] = Math.min(max, Math.max(min, nextSizes[i]));
      remaining -= nextSizes[i];
    }

    if (remaining >= 0 && nextSizes.length > 0) {
      const last = nextSizes.length - 1;
      nextSizes[last] = Math.max(panes[last]?.size?.minPx ?? 120, nextSizes[last] + remaining);
    } else if (remaining < 0) {
      let deficit = -remaining;
      for (let i = nextSizes.length - 1; i >= 0 && deficit > 0; i -= 1) {
        const min = mins[i] ?? 0;
        const reducible = Math.max(0, nextSizes[i] - min);
        const reduce = Math.min(deficit, reducible);
        nextSizes[i] -= reduce;
        deficit -= reduce;
      }
      
      // ... (Rest of distribution logic) ...
      if (deficit > 0) {
        const total = nextSizes.reduce((acc, value) => acc + value, 0);
        if (total > 0) {
          const ratio = width / total;
          for (let i = 0; i < nextSizes.length; i += 1) {
            nextSizes[i] = Math.max(32, Math.floor(nextSizes[i] * ratio));
          }
          if (nextSizes.length > 0) {
            const last = nextSizes.length - 1;
            const sum = nextSizes.slice(0, last).reduce((acc, value) => acc + value, 0);
            nextSizes[last] = Math.max(32, width - sum);
          }
        }
      }
    }

    // Write back to Map if changed
    if (nextSizes.some((value, index) => value !== currentSizes[index])) {
      setSizeMap(prev => {
        const nextMap = { ...prev };
        panes.forEach((pane, i) => {
          nextMap[pane.key] = nextSizes[i];
        });
        return nextMap;
      });
    }
  };

  const handleDrag = (index: number, clientX: number, containerLeft: number, containerWidthValue: number) => {
    const panes = getPanes();
    const current = sizes(); // Get current array
    
    // ... (Keep existing drag calculation logic) ...
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

    // Write directly to Map
    setSizeMap(prev => ({
      ...prev,
      [panes[index].key]: nextLeft,
      [panes[index + 1].key]: nextRight
    }));
  };

  createEffect(clampSizes);

  // Sync: When panes change, initialize new keys, keep old ones
  createEffect(on(getPanes, (panes) => {
    setSizeMap(prev => {
      const next: Record<string, number> = {};
      let changed = false;
      
      // Only keep keys that are currently present
      panes.forEach(pane => {
        if (prev[pane.key] !== undefined) {
          next[pane.key] = prev[pane.key];
        } else {
          next[pane.key] = pane.size?.initialPx ?? 300;
          changed = true;
        }
      });

      // Check if any keys were removed (length mismatch)
      if (!changed && Object.keys(prev).length !== Object.keys(next).length) {
        changed = true;
      }

      return changed ? next : prev;
    });
  }));

  return {
    sizes,
    handleDrag
  };
};
