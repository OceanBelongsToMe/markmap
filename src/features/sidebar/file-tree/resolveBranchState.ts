import type { StateIconState } from "../../../ui/icons/state/types";

export const resolveBranchState = (isExpanded: boolean): StateIconState =>
  isExpanded ? "open" : "closed";
