import { createMemo } from "solid-js";
import { taxonomy } from "../../../ia/taxonomy";
import { useI18n } from "../../../i18n/context";

export type EditorOption = {
  value: string;
  label: string;
};

export const useEditorSettings = () => {
  const { t } = useI18n();

  const contentOptions = createMemo<EditorOption[]>(() =>
    taxonomy.contentTypes.map((type) => ({
      value: type,
      label:
        type === "note"
          ? t("contentTypeNote")
          : type === "outline"
            ? t("contentTypeOutline")
            : t("contentTypeMarkmap")
    }))
  );

  const inputOptions = createMemo<EditorOption[]>(() =>
    taxonomy.editorInputs.map((mode) => ({
      value: mode,
      label:
        mode === "markdown-text"
          ? t("editorInputMarkdownText")
          : mode === "wysiwyg"
            ? t("editorInputWysiwyg")
            : t("editorInputMarkmap")
    }))
  );

  return {
    contentOptions,
    inputOptions
  };
};
