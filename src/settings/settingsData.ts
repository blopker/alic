import { settingsChangedListener } from "@/listeners";
import { createStore } from "solid-js/store";
import { type ProfileData, type SettingsData, commands } from "../bindings";

const [settings, setSettings] = createStore<SettingsData>(await getSettings());

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
  // console.log(opt.data);
  return opt.data;
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
  resetSettings,
  updateProfile,
  deleteProfile,
  createProfile,
  setProfileActive,
  getProfileActive,
};
