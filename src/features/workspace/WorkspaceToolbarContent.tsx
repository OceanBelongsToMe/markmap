import { createMemo } from "solid-js";
import { useI18n } from "../../i18n/context";
import type { Locale } from "../../i18n";
import { Label } from "../../ui/components/Label";
import { Select } from "../../ui/components/Select";
import type { FileTreeStyle } from "../sidebar/file-tree";

export type WorkspaceToolbarContentProps = {
  fileTreeStyle: FileTreeStyle;
  onFileTreeStyleChange: (style: FileTreeStyle) => void;
};

export const WorkspaceToolbarContent = (props: WorkspaceToolbarContentProps) => {
  const { locale, setLocale, supportedLocales, t } = useI18n();
  const options = createMemo(() =>
    supportedLocales.map((value) => ({
      value,
      label: value === "en" ? t("languageEnglish") : t("languageZhCN")
    }))
  );

  const styleOptions = [
    { value: "ark", label: "ark" },
    { value: "dense", label: "dense" },
    { value: "classic", label: "classic" }
  ];

  return (
    <>
      <div style={{ display: "flex", "align-items": "center", gap: "12px" }}>
        <div style={{ display: "flex", "align-items": "center", gap: "8px" }}>
          <Label for="locale-select" text={t("language")} />
          <Select
            id="locale-select"
            value={locale()}
            options={options()}
            onChange={(value) => setLocale(value as Locale)}
          />
        </div>
        <div style={{ width: "1px", height: "16px", background: "var(--color-border-subtle)" }} />
        <div style={{ display: "flex", "align-items": "center", gap: "8px" }}>
          <Label for="style-select" text="Tree Style" />
          <Select
            id="style-select"
            value={props.fileTreeStyle}
            options={styleOptions}
            onChange={(value) => props.onFileTreeStyleChange(value as FileTreeStyle)}
          />
        </div>
      </div>
    </>
  );
};