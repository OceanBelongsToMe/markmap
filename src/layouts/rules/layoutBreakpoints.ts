export type LayoutVariant = "three-pane" | "two-pane" | "single-pane";

export type LayoutMinSizes = {
  sideMin: number;
  editorMin: number;
  previewMin: number;
};

export const defaultLayoutMins: LayoutMinSizes = {
  sideMin: 180,
  editorMin: 480,
  previewMin: 300
};

export const getLayoutVariant = (width: number, mins: LayoutMinSizes): LayoutVariant => {
  if (width <= 0) return "three-pane";

  const threePaneMin = mins.sideMin + mins.editorMin + mins.previewMin;
  const twoPaneMin = mins.sideMin + mins.editorMin;

  if (width >= threePaneMin) return "three-pane";
  if (width >= twoPaneMin) return "two-pane";
  return "single-pane";
};
