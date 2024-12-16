import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { A, useNavigate } from "@solidjs/router";
import {
  type Component,
  For,
  type JSXElement,
  createEffect,
  createMemo,
  createSignal,
} from "solid-js";
import { ConfirmModal, confirmModal } from "./ConfirmModal";
import {
  SettingBox,
  SettingRow,
  SettingsButton,
  SettingsInput,
  SettingsNumberInput,
  SettingsPage,
} from "./SettingsUI";
import {
  createProfile,
  resetSettings,
  setThreads,
  settings,
} from "./settingsData";

interface SettingsPageData {
  kind: string;
  title: string;
  id?: number;
  page?: Component;
}

const [settingsPages, _] = createSignal<SettingsPageData[]>([
  { kind: "general", title: "General", page: GeneralPage },
]);

// const themeKinds: ThemeKind[] = ["System", "Light", "Dark"];

function Settings(props: { children?: JSXElement }) {
  return (
    <main class="flex h-screen w-full justify-between bg-secondary">
      <ConfirmModal />
      <div class="w-40 overflow-y-auto border-accent border-r-[1px]">
        <SettingsSideBar />
      </div>
      <div class="grow bg-primary">{props.children}</div>
    </main>
  );
}

function SettingsSideBar() {
  const profilePages = createMemo<SettingsPageData[]>(() => {
    return settings.profiles.map((p) => ({
      kind: `profile:${p.name}`,
      title: p.name,
      id: p.id,
    }));
  });
  return (
    <div class="flex flex-col items-start gap-2 p-4">
      <For each={settingsPages()}>
        {(p) => (
          <A activeClass="font-bold" href="/settings" end>
            {p.title}
          </A>
        )}
      </For>
      <div class="opacity-70">Profiles</div>
      <div class="flex flex-col items-start gap-2 pl-2">
        <For each={profilePages()}>
          {(p) => (
            <A activeClass="font-bold" href={`/settings/profile/${p.id}`}>
              {p.title}
            </A>
          )}
        </For>
        <A activeClass="font-bold" href="/settings/newprofile">
          New Profile...
        </A>
      </div>
    </div>
  );
}

function GeneralPage() {
  return (
    <SettingsPage title="General">
      <SettingBox title="">
        <SettingRow
          title="Threads"
          helpText="Number of images to process in parallel. Setting this to 0 will use all available cores."
        >
          <SettingsNumberInput
            value={settings.threads || 0}
            onChange={(value) => {
              setThreads(value);
            }}
          />
        </SettingRow>
        <SettingRow title="Reset All Settings">
          <SettingsButton
            onClick={async () => {
              confirmModal({
                text: "Are you sure you want to reset all settings?",
                onConfirm: resetSettings,
              });
            }}
            style="danger"
          >
            Reset
          </SettingsButton>
        </SettingRow>
      </SettingBox>
    </SettingsPage>
  );
}

function NewProfilePage() {
  const [newProfileName, setNewProfileName] = createSignal("");
  const navigate = useNavigate();
  const event = useKeyDownEvent();
  createEffect(() => {
    const e = event();
    if (e && e.key === "Enter") {
      onOK();
    }
  });
  async function onOK() {
    const name = newProfileName().substring(0, 30);
    if (name === "") {
      return;
    }
    await createProfile(name);
    setNewProfileName("");
    const newProfile = settings.profiles.find((p) => p.name === name);
    if (newProfile) {
      navigate(`/settings/profile/${newProfile.id}`);
    }
  }
  return (
    <SettingsPage title="Create New Profile">
      <SettingBox title="">
        <SettingRow title="Profile Name">
          <SettingsInput
            autoFocus={true}
            placeholder="Name"
            label="Name"
            value=""
            onChange={async (value) => {
              setNewProfileName(value);
            }}
          />
        </SettingRow>
        <SettingsButton disabled={newProfileName() === ""} onClick={onOK}>
          Create
        </SettingsButton>
      </SettingBox>
    </SettingsPage>
  );
}

export { Settings, GeneralPage, NewProfilePage };
