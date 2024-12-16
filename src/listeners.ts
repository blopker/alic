import { listen } from "@tauri-apps/api/event";

function openFileDialogListener(cb: () => void) {
  return listen("open-find-image-dialog", () => {
    cb();
  });
}

function addFileListener(cb: (path: string) => void) {
  return listen<string>("add-file", (event) => {
    cb(event.payload);
  });
}

function clearFilesListener(cb: () => void) {
  return listen("clear-files", (_) => {
    cb();
  });
}

function settingsChangedListener(cb: () => void) {
  listen("settings-changed", (_) => {
    cb();
  });
}

export {
  openFileDialogListener,
  addFileListener,
  clearFilesListener,
  settingsChangedListener,
};
