import { labels } from "../ia/labels";
import { supportedLocales, type SupportedLocale } from "./supported-locales";
import { zhCN } from "./zh-CN";

export type Locale = SupportedLocale;
export type LabelKey = keyof typeof labels;

type Dictionary = Record<LabelKey, string>;

const dictionaries: Partial<Record<Locale, Dictionary>> = {
  "zh-CN": zhCN
};

export const translate = (key: LabelKey, locale: Locale = "en") => {
  if (locale === "en") {
    return labels[key];
  }

  return dictionaries[locale]?.[key] ?? labels[key];
};

export const getSupportedLocales = () => supportedLocales;
