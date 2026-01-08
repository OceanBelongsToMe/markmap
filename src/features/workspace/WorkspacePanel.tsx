import { useI18n } from "../../i18n/context";
import { EditorSettings } from "./EditorSettings";
import { WorkspacePanelView } from "./components/WorkspacePanelView";
import { useWorkspaceActions } from "./hooks/useWorkspaceActions";

export const WorkspacePanel = () => {
  const { t } = useI18n();
  const { importFolder } = useWorkspaceActions();

  return (
    <>
      <WorkspacePanelView
        strings={{
          welcomeTitle: t("welcomeTitle"),
          learnMore: t("learnMore"),
          greet: t("greet"),
          enterName: t("enterName")
        }}
        name={() => ""}
        onNameChange={() => {}}
        greetMsg={() => ""}
        onGreet={importFolder}
      />
      <EditorSettings />
    </>
  );
};
