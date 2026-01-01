import { createContext, createSignal, useContext, type Accessor, type JSX, type Setter } from "solid-js";
import { getSupportedLocales, translate, type LabelKey, type Locale } from "./index";

type I18nContextValue = {
  locale: Accessor<Locale>;
  setLocale: Setter<Locale>;
  t: (key: LabelKey) => string;
  supportedLocales: readonly Locale[];
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

  return (
    <I18nContext.Provider value={{ locale, setLocale, t, supportedLocales }}>
      {props.children}
    </I18nContext.Provider>
  );
};

export const useI18n = () => {
  const context = useContext(I18nContext);
  if (!context) {
    throw new Error("useI18n must be used within I18nProvider");
  }
  return context;
};
