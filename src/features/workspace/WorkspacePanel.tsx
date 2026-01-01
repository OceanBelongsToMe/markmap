import { useI18n } from "../../i18n/context";
import { EditorSettings } from "./EditorSettings";
import { WorkspacePanelView } from "./components/WorkspacePanelView";
import { useWorkspaceActions } from "./hooks/useWorkspaceActions";

export const WorkspacePanel = () => {
  const { t } = useI18n();
  const { greetMsg, name, setName, greet } = useWorkspaceActions();

  return (
    <>
      <WorkspacePanelView
        strings={{
          welcomeTitle: t("welcomeTitle"),
          learnMore: t("learnMore"),
          greet: t("greet"),
          enterName: t("enterName")
        }}
        name={name}
        onNameChange={setName}
        greetMsg={greetMsg}
        onGreet={greet}
      />
      <EditorSettings />
    </>
  );
};
