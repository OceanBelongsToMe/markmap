export type FocusTrapOptions = {
  initialFocus?: HTMLElement | null;
  onEscape?: () => void;
};

const focusableSelector =
  "a[href], button, input, textarea, select, [tabindex]:not([tabindex='-1'])";

export const trapFocus = (container: HTMLElement, options: FocusTrapOptions = {}) => {
  const focusables = Array.from(
    container.querySelectorAll<HTMLElement>(focusableSelector)
  ).filter((el) => !el.hasAttribute("disabled") && el.tabIndex !== -1);

  const first = focusables[0] ?? null;
  const last = focusables[focusables.length - 1] ?? null;

  const focusTarget = options.initialFocus ?? first;
  focusTarget?.focus();

  const onKeyDown = (event: KeyboardEvent) => {
    if (event.key === "Escape") {
      options.onEscape?.();
      return;
    }

    if (event.key !== "Tab" || !first || !last) return;

    if (event.shiftKey && document.activeElement === first) {
      event.preventDefault();
      last.focus();
      return;
    }

    if (!event.shiftKey && document.activeElement === last) {
      event.preventDefault();
      first.focus();
    }
  };

  container.addEventListener("keydown", onKeyDown);

  return () => container.removeEventListener("keydown", onKeyDown);
};
