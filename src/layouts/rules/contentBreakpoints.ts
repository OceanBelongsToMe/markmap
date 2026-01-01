export type ContentVariant = "full" | "compact" | "minimal";

export type ContentBreakpoints = {
  compact: number;
  minimal: number;
};

export const defaultContentBreakpoints: ContentBreakpoints = {
  compact: 960,
  minimal: 720
};

export const getContentVariant = (
  width: number,
  breakpoints: ContentBreakpoints
): ContentVariant => {
  if (width <= 0) return "full";
  if (width >= breakpoints.compact) return "full";
  if (width >= breakpoints.minimal) return "compact";
  return "minimal";
};
