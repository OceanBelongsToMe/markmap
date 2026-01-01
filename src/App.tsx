import { Route, Router } from "@solidjs/router";
import { I18nProvider } from "./i18n/context";
import { routes } from "./routes";
import "./App.css";
import "./ui/typography/typography.css";
import "./ui/theme/theme.css";

function App() {
  return (
    <I18nProvider>
      <Router>
        {routes.map((route) => (
          <Route path={route.path} component={route.component} />
        ))}
      </Router>
    </I18nProvider>
  );
}

export default App;
