export type FocusRestoreOptions = {
  fallback?: HTMLElement | null;
};

export const captureFocus = () => {
  return document.activeElement instanceof HTMLElement ? document.activeElement : null;
};

export const restoreFocus = (
  target: HTMLElement | null,
  options: FocusRestoreOptions = {}
) => {
  const next = target ?? options.fallback ?? null;
  next?.focus();
};
