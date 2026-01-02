import { Route, Router } from "@solidjs/router";
import { I18nProvider } from "./i18n/context";
import { routes } from "./routes";
import { StableList } from "./ui/components/StableList";
import "./App.css";
import "./ui/typography/typography.css";
import "./ui/theme/theme.css";
import "./layouts/sidebar.css";

function App() {
  return (
    <I18nProvider>
      <Router>
        <StableList each={() => routes}>
          {(route) => (
            <Route path={route().path} component={route().component} />
          )}
        </StableList>
      </Router>
    </I18nProvider>
  );
}

export default App;
