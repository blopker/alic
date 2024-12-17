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

function updateResultListener(cb: (result: string) => void) {
  listen<string>("update-result", (result) => {
    cb(result.payload);
  });
}

export {
  openFileDialogListener,
  addFileListener,
  clearFilesListener,
  settingsChangedListener,
  updateResultListener,
};
