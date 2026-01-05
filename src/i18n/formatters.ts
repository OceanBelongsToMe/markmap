import { toArkLocale } from "./locale";

type DateFormatterOptions = Intl.DateTimeFormatOptions;
type NumberFormatterOptions = Intl.NumberFormatOptions;

type FormatDate = (date: Date, options?: DateFormatterOptions) => string;
type FormatNumber = (value: number, options?: NumberFormatterOptions) => string;
type FormatCurrency = (value: number, currency: string, options?: NumberFormatterOptions) => string;

export const createFormatters = (locale: string) => {
  const normalizedLocale = toArkLocale(locale);

  const formatDate: FormatDate = (date, options) =>
    new Intl.DateTimeFormat(normalizedLocale, options).format(date);

  const formatNumber: FormatNumber = (value, options) =>
    new Intl.NumberFormat(normalizedLocale, options).format(value);

  const formatCurrency: FormatCurrency = (value, currency, options) =>
    new Intl.NumberFormat(normalizedLocale, {
      style: "currency",
      currency,
      ...(options ?? {})
    }).format(value);

  return { formatDate, formatNumber, formatCurrency };
};
