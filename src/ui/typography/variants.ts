export type TypographyVariant =
  | "h1"
  | "h2"
  | "h3"
  | "body"
  | "caption"
  | "code"
  | "comment";

export const typographyVariantClass: Record<TypographyVariant, string> = {
  h1: "typo-h1",
  h2: "typo-h2",
  h3: "typo-h3",
  body: "typo-body",
  caption: "typo-caption",
  code: "typo-code",
  comment: "typo-comment"
};
