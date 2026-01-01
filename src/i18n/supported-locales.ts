export const supportedLocales = ["en", "zh-CN"] as const;

export type SupportedLocale = (typeof supportedLocales)[number];
