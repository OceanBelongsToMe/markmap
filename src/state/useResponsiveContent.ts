import { createMemo } from "solid-js";
import {
  defaultContentBreakpoints,
  getContentVariant,
  type ContentBreakpoints
} from "../layouts/rules/contentBreakpoints";
import { useMeasuredWidth } from "./useMeasuredWidth";

export const useResponsiveContent = (
  getElement: () => HTMLElement | undefined,
  breakpoints: ContentBreakpoints = defaultContentBreakpoints
) => {
  const width = useMeasuredWidth(getElement);
  const contentVariant = createMemo(() => getContentVariant(width(), breakpoints));

  return {
    contentVariant,
    width
  };
};
