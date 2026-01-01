import { EditorSettingsView } from "./components/EditorSettingsView";
import { useEditorSettings } from "./hooks/useEditorSettings";

export const EditorSettings = () => {
  const { contentOptions, inputOptions } = useEditorSettings();

  return <EditorSettingsView contentOptions={contentOptions} inputOptions={inputOptions} />;
};
