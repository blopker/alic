import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { A, useNavigate } from "@solidjs/router";
import { IoAdd } from "solid-icons/io";
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

function SettingsLink(props: {
  href: string;
  children: JSXElement;
  end?: boolean;
}) {
  return (
    <A
      activeClass="bg-indigo-600 text-white font-medium rounded-md"
      inactiveClass="hover:bg-accent transition-colors"
      class="mb-1 block rounded-md px-3 py-1.5 text-sm"
      href={props.href}
      end={props.end}
    >
      {props.children}
    </A>
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
    <div class="flex h-full flex-col bg-secondary py-4">
      <div class="px-4 pb-6">
        <For each={settingsPages()}>
          {(p) => (
            <SettingsLink href="/settings" end>
              {p.title}
            </SettingsLink>
          )}
        </For>
      </div>

      <div class="px-4">
        <div class="mb-2 px-3 font-medium text-xs uppercase tracking-wide opacity-70">
          Profiles
        </div>
        <div class="flex flex-col">
          <For each={profilePages()}>
            {(p) => (
              <SettingsLink href={`/settings/profile/${p.id}`}>
                {p.title}
              </SettingsLink>
            )}
          </For>
          <SettingsLink href="/settings/newprofile">
            <div class="flex items-center gap-1">
              <IoAdd />
              New Profile
            </div>
          </SettingsLink>
        </div>
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
