import type { StateIconContext, StateIconState, StateIconTheme } from "./types";
import { resolveStateIcon } from "./resolveStateIcon";

export type StateIconProps = {
  context: StateIconContext;
  state: StateIconState;
  theme?: StateIconTheme;
  class?: string;
};

export const StateIcon = (props: StateIconProps) => {
  return (
    <span
      class={props.class}
      aria-hidden="true"
      style={{ display: "inline-flex", "align-items": "center", "line-height": "0" }}
    >
      {resolveStateIcon({
        context: props.context,
        state: props.state,
        theme: props.theme
      })}
    </span>
  );
};
