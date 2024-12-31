import { open } from "@tauri-apps/plugin-dialog";
import { FaSolidXmark } from "solid-icons/fa";
import { VsAdd, VsSettings } from "solid-icons/vs";
import { createSignal, type JSXElement, onMount, Show } from "solid-js";
import { type ProfileData, commands } from "./bindings";
import { FILE_TYPES } from "./constants";
import { openFileDialogListener } from "./listeners";
import { SettingsSelect } from "./settings/SettingsUI";
import {
  getProfileActive,
  setProfileActive,
  settings,
} from "./settings/settingsData";
import { addFile, clearFiles, store } from "./store";
import { toHumanReadableSize } from "./utils";
import { emit } from "@tauri-apps/api/event";
import {
  onImageUpdate,
  listenToMonitorStatusUpdate,
  onImageBinaryUpdate,
  onFilesUpdate,
  isMonitorRunning,
  startListening,
  stopMonitor,
} from "tauri-plugin-clipboard-api";

openFileDialogListener(() => {
  openFile();
});

async function openFile() {
  const file = await open({
    multiple: true,
    directory: false,
    filters: [
      {
        name: "Images",
        extensions: FILE_TYPES,
      },
    ],
  });
  if (!file) {
    return;
  }
  for (const f of file) {
    addFile(f);
  }
}

export default function BottomBar() {
  const options = () => {
    const options: Array<{ label: string; value: ProfileData | null }> =
      settings.profiles.map((p) => {
        return {
          label: p.name,
          value: p,
        };
      });
    options.push({
      label: "New Profile...",
      value: null,
    });
    return options;
  };
  const [monitorRunning, setMonitorRunning] = createSignal(false);
  onMount(async () => {
    const unlisten = onImageUpdate((img) => {
      emit("pasteImage");
      // stop monitor and start again
    });
    const unlisten4 = onFilesUpdate((_) => {
      console.log("file");
    });

    const unlisten2 = onImageBinaryUpdate((_) => {
      console.log("binary");
      emit("pastImage");
    });
    const unlisten3 = listenToMonitorStatusUpdate((running) => {
      setMonitorRunning(running);
    });

    return [unlisten, unlisten2, unlisten3, unlisten4];
  });
  return (
    <div class="right-0 left-0 flex h-10 items-center justify-between gap-2 border-accent border-t-[1px] bg-secondary px-2">
      <AddButton />
      <Show
        when={monitorRunning()}
        fallback={
          <Button
            onClick={async () => {
              await startListening({ image: true }).catch((e) =>
                console.error(e)
              );
            }}
          >
            Start+
          </Button>
        }
      >
        Running
      </Show>
      <StatusText />
      <span class="grow" />
      <SettingsSelect
        value={getProfileActive().name}
        bgColor="bg-primary"
        class="w-40"
        onChange={(label) => {
          const option = options().find((o) => o.label === label);
          if (option?.value === null) {
            commands.openSettingsWindow("/settings/newprofile");
            setProfileActive(getProfileActive().id);
            return;
          }
          const profile = option?.value;
          if (profile) {
            setProfileActive(profile.id);
          }
        }}
        options={options().map((e) => e.label)}
      />
      <SettingsButton />
      <ClearButton />
    </div>
  );
}

function AddButton() {
  return (
    <Button onClick={openFile}>
      <span class="flex items-center justify-center text-sm">
        <VsAdd />
      </span>
    </Button>
  );
}

function ClearButton() {
  return (
    <Button onClick={clearFiles} disabled={store.files.length === 0}>
      <span class="flex items-center gap-1 px-2 text-sm">
        <FaSolidXmark /> Clear
      </span>
    </Button>
  );
}

async function settingsWindow() {
  await commands.openSettingsWindow(
    `/settings/profile/${getProfileActive().id}`
  );
}

function StatusText() {
  const doneFiles = () => store.files.filter((f) => f.status === "Complete");
  const dataSaved = () =>
    doneFiles()
      .map((f) => f.originalSize ?? 0 - (f.size ?? 0))
      .reduce((a, b) => a + b, 0);
  const dataSavedPercent = () =>
    doneFiles()
      .filter((f) => f.savings)
      .map((f) => f.savings ?? 0)
      .reduce((a, b) => a + b, 0) / doneFiles().length;
  return (
    <Show when={doneFiles().length > 0}>
      {toHumanReadableSize(dataSaved())} saved, average{" "}
      {dataSavedPercent().toFixed(1)}%
    </Show>
  );
}

function SettingsButton() {
  return (
    <Button onClick={settingsWindow}>
      <span class="flex items-center justify-center text-sm">
        <VsSettings />
      </span>
    </Button>
  );
}

function Button(props: {
  onClick: () => void;
  children: JSXElement;
  class?: string;
  disabled?: boolean;
}) {
  return (
    <button
      type="button"
      onClick={props.onClick}
      disabled={props.disabled}
      class={`${props.class} relative m-0 min-h-6 min-w-10 rounded-sm border-[0.5px] border-accent p-0 text-center leading-none transition-all enabled:hover:bg-gray-600 disabled:cursor-not-allowed disabled:opacity-50`}
    >
      {props.children}
    </button>
  );
}
