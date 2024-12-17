import { events } from "./bindings";

function openFileDialogListener(cb: () => void) {
  return events.openAddFileDialogEvent.listen(() => {
    cb();
  });
}

function addFileListener(cb: (path: string) => void) {
  return events.addFileEvent.listen((event) => {
    cb(event.payload);
  });
}

function clearFilesListener(cb: () => void) {
  return events.clearFilesEvent.listen(() => {
    cb();
  });
}

function settingsChangedListener(cb: () => void) {
  return events.settingsChangedEvent.listen(() => {
    cb();
  });
}

function updateResultListener(cb: (result: string) => void) {
  return events.updateResultsEvent.listen((event) => {
    cb(event.payload);
  });
}

export {
  openFileDialogListener,
  addFileListener,
  clearFilesListener,
  settingsChangedListener,
  updateResultListener,
};
