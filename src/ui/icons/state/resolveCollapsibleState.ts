import type { StateIconState } from "./types";

export const resolveCollapsibleState = (isOpen: boolean): StateIconState =>
  isOpen ? "open" : "closed";
