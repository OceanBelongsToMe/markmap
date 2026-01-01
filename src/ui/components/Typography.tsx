import type { JSX } from "solid-js";
import { Dynamic } from "solid-js/web";
import { typographyVariantClass, type TypographyVariant } from "../typography/variants";

export type TypographyProps = {
  variant: TypographyVariant;
  as?: keyof JSX.IntrinsicElements;
  class?: string;
  children: JSX.Element;
};

export const Typography = (props: TypographyProps) => {
  const Tag = props.as ?? "p";
  const className = props.class
    ? `${typographyVariantClass[props.variant]} ${props.class}`
    : typographyVariantClass[props.variant];

  return <Dynamic component={Tag} class={className}>{props.children}</Dynamic>;
};
