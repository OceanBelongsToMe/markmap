export type TextDirection = "ltr" | "rtl";

const RTL_LOCALE_PREFIXES = new Set(["ar", "he", "fa", "ur"]);

export const toArkLocale = (locale: string) => {
  if (locale === "en") {
    return "en-US";
  }

  return locale;
};

export const getTextDirection = (locale: string): TextDirection => {
  const normalized = toArkLocale(locale);
  const prefix = normalized.split("-")[0] ?? normalized;
  return RTL_LOCALE_PREFIXES.has(prefix) ? "rtl" : "ltr";
};
