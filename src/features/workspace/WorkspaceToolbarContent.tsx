import { createMemo } from "solid-js";
import { useI18n } from "../../i18n/context";
import type { Locale } from "../../i18n";
import { Label } from "../../ui/components/Label";
import { Select } from "../../ui/components/Select";

export const WorkspaceToolbarContent = () => {
  const { locale, setLocale, supportedLocales, t } = useI18n();
  const options = createMemo(() =>
    supportedLocales.map((value) => ({
      value,
      label: value === "en" ? t("languageEnglish") : t("languageZhCN")
    }))
  );

  return (
    <>
      <Label for="locale-select" text={t("language")} />
      <Select
        id="locale-select"
        value={locale()}
        options={options()}
        onChange={(value) => setLocale(value as Locale)}
      />
    </>
  );
};
