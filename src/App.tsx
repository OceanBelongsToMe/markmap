import { Route, Router } from "@solidjs/router";
import { I18nProvider, useI18n } from "./i18n/context";
import { routes } from "./routes";
import { StableList } from "./ui/components/StableList";
import "./App.css";
import "./ui/components/collapsible.css";
import "./ui/styles/tree-view.css";
import "./ui/typography/typography.css";
import "./ui/theme/theme.css";
import "./layouts/sidebar.css";

const AppShell = () => {
  const { direction } = useI18n();

  return (
    <div dir={direction()}>
      <Router>
        <StableList each={() => routes}>
          {(route) => (
            <Route path={route().path} component={route().component} />
          )}
        </StableList>
      </Router>
    </div>
  );
};

function App() {
  return (
    <I18nProvider>
      <AppShell />
    </I18nProvider>
  );
}

export default App;
