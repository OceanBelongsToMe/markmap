import { LocaleProvider } from "@ark-ui/solid/locale";
import { createContext, createSignal, useContext, type Accessor, type JSX, type Setter } from "solid-js";
import { createFormatters } from "./formatters";
import { getSupportedLocales, translate, type LabelKey, type Locale } from "./index";
import { getTextDirection, type TextDirection, toArkLocale } from "./locale";

type I18nContextValue = {
  locale: Accessor<Locale>;
  setLocale: Setter<Locale>;
  t: (key: LabelKey) => string;
  supportedLocales: readonly Locale[];
  direction: Accessor<TextDirection>;
  formatDate: (date: Date, options?: Intl.DateTimeFormatOptions) => string;
  formatNumber: (value: number, options?: Intl.NumberFormatOptions) => string;
  formatCurrency: (value: number, currency: string, options?: Intl.NumberFormatOptions) => string;
};

const I18nContext = createContext<I18nContextValue>();

export type I18nProviderProps = {
  children: JSX.Element;
  defaultLocale?: Locale;
};

export const I18nProvider = (props: I18nProviderProps) => {
  const [locale, setLocale] = createSignal<Locale>(props.defaultLocale ?? "en");
  const t = (key: LabelKey) => translate(key, locale());
  const supportedLocales = getSupportedLocales();
  const direction = () => getTextDirection(locale());
  const formatters = () => createFormatters(locale());

  return (
    <LocaleProvider locale={toArkLocale(locale())}>
      <I18nContext.Provider
        value={{
          locale,
          setLocale,
          t,
          supportedLocales,
          direction,
          formatDate: (date, options) => formatters().formatDate(date, options),
          formatNumber: (value, options) => formatters().formatNumber(value, options),
          formatCurrency: (value, currency, options) =>
            formatters().formatCurrency(value, currency, options)
        }}
      >
        {props.children}
      </I18nContext.Provider>
    </LocaleProvider>
  );
};

export const useI18n = () => {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error("useI18n must be used within I18nProvider");
  }
  return context;
};
