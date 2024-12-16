import { HashRouter, Route } from "@solidjs/router";
import App from "./App";
import { ProfilePage } from "./settings/ProfilePage";
import { GeneralPage, NewProfilePage, Settings } from "./settings/SettingsPage";

export function AppRouter() {
  return (
    <HashRouter>
      <Route path="/" component={App} />
      <Route path="/settings" component={Settings}>
        <Route path="/" component={GeneralPage} />
        <Route path="/profile/:profileid" component={ProfilePage} />
        <Route path="/newprofile" component={NewProfilePage} />
      </Route>
    </HashRouter>
  );
}
