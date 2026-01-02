export type ChevronIconProps = {
  class?: string;
};

export const ChevronIcon = (props: ChevronIconProps) => {
  return (
    <svg
      class={props.class}
      viewBox="0 0 16 16"
      aria-hidden="true"
      focusable="false"
    >
      <polyline points="3,6 8,11 13,6" />
    </svg>
  );
};
