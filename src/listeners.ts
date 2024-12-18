import { events, type UpdateStateEvent } from "./bindings";

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

function badFileListener(cb: (path: string) => void) {
  return events.badFileEvent.listen((event) => {
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

function updateResultListener(cb: (result: UpdateStateEvent) => void) {
  return events.updateStateEvent.listen((event) => {
    cb(event.payload);
  });
}

export {
  openFileDialogListener,
  addFileListener,
  clearFilesListener,
  settingsChangedListener,
  updateResultListener,
  badFileListener,
};
