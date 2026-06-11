import { createStore } from "solid-js/store";
import { settingsChangedListener } from "@/listeners";
// The backend always sends fully-populated settings, so use the _Serialize
// shapes (all fields required) rather than the _Deserialize unions.
import {
  commands,
  type ProfileData_Serialize as ProfileData,
  type SettingsData_Serialize as SettingsData,
} from "../bindings";
import { addToast } from "../Toast";

const [settings, setSettings] = createStore<SettingsData>(await getSettings());

// On startup, switch to the default profile if one is configured.
// Reading the store outside a tracked scope is intentional: this must run
// exactly once and should not re-run when settings change.
/* oxlint-disable solid/reactivity */
if (settings.default_profile_id !== null) {
  const defaultExists = settings.profiles.some(
    (p) => p.id === settings.default_profile_id,
  );
  if (defaultExists) {
    setProfileActive(settings.default_profile_id);
  }
}
/* oxlint-enable solid/reactivity */

settingsChangedListener(async () => {
  // console.log("settings changed");
  setSettings(await getSettings());
});

function debounce<F extends (...args: Parameters<F>) => ReturnType<F>>(
  func: F,
  waitFor: number,
): (...args: Parameters<F>) => void {
  let timeout: ReturnType<typeof setTimeout>;

  return (...args: Parameters<F>): void => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), waitFor);
  };
}

async function getSettings() {
  const opt = await commands.getSettings();
  if (opt.status === "error") {
    throw new Error(opt.error);
  }
  if (opt.data.warning) {
    addToast({ message: opt.data.warning, type: "warning", duration: 0 });
  }
  return opt.data.settings;
}

async function resetSettings() {
  await commands.resetSettings();
}

async function saveSettings() {
  await commands.saveSettings(settings);
}

const debounceSaveSettings = debounce(saveSettings, 500);

function setThreads(threads: SettingsData["threads"]) {
  let _threads = threads || 0;
  if (_threads < 0) {
    _threads = 0;
  }
  setSettings("threads", threads);
  saveSettings();
}

function setDefaultProfileId(id: number | null) {
  setSettings("default_profile_id", id);
  saveSettings();
}

function updateProfile(profileid: number, update: Partial<ProfileData>) {
  const profileIdx = settings.profiles.findIndex((p) => p.id === profileid);
  if (profileIdx === -1) {
    return;
  }
  const profile = settings.profiles[profileIdx];
  if (profile.id !== profileid) {
    return;
  }
  setSettings("profiles", profileIdx, {
    ...profile,
    ...update,
  });
  debounceSaveSettings();
}

async function deleteProfile(profileid: number) {
  await commands.deleteProfile(profileid);
}

async function createProfile(name: string) {
  await commands.addProfile(name);
  setSettings(await getSettings());
}

async function setProfileActive(profileid: number) {
  let found = false;
  for (const profile of settings.profiles) {
    if (profile.id === profileid) {
      updateProfile(profileid, { active: true });
      found = true;
    } else {
      updateProfile(profile.id, { active: false });
    }
  }
  if (!found) {
    updateProfile(0, { active: true });
  }
}

function getProfileActive() {
  return settings.profiles.find((p) => p.active) || settings.profiles[0];
}

export {
  settings,
  setThreads,
  setDefaultProfileId,
  resetSettings,
  updateProfile,
  deleteProfile,
  createProfile,
  setProfileActive,
  getProfileActive,
};
