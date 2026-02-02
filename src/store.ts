import { createStore } from "solid-js/store";
import { commands, type FileEntry } from "./bindings";
import { compressImage } from "./compress";
import { clearFilesListener } from "./listeners";
import { getProfileActive, settings } from "./settings/settingsData";
import { Semaphore } from "./utils";

clearFilesListener(clearFiles);

const CPU_COUNT = await commands.getCpuCount();
const semaphore = new Semaphore(0);
syncSemaphore();

type ReadonlyFileEntry = Readonly<FileEntry>;

interface Store {
  files: ReadonlyFileEntry[];
}

const [store, setStore] = createStore<Store>({
  files: [],
});

function syncSemaphore() {
  semaphore.maxConcurrent = settings.threads || CPU_COUNT;
}

function newFileEntry(
  path: string,
  data: Partial<FileEntry>,
): ReadonlyFileEntry {
  return {
    path,
    file: data.file ?? "",
    status: data.status ?? "Processing",
    size: data.size ?? null,
    originalSize: data.originalSize ?? null,
    ext: data.ext ?? "",
    error: data.error ?? null,
    savings: data.savings ?? null,
  };
}

async function addFile(path: string) {
  if (store.files.find((f) => f.path === path)) {
    return;
  }

  let file = newFileEntry(path, {});
  setStore("files", (f) => [...f, file]);

  const fileResult = await commands.getFileInfo(path);
  if (fileResult.status === "error") {
    console.log(fileResult.error);
    updateFile(file, { error: fileResult.error, status: "Error" });
    return;
  }

  const update: Partial<FileEntry> = {
    file: fileResult.data.filename,
    ext: fileResult.data.extension,
    originalSize: fileResult.data.size,
  };
  file = updateFile(file, update);
  updateDockBadge();
  await semaphore.acquire();
  syncSemaphore();
  try {
    await compressFile(file);
  } finally {
    semaphore.release();
    updateDockBadge();
  }
}

async function compressFile(_file: FileEntry) {
  let file = updateFile(_file, { status: "Compressing" });
  const parallelImages = semaphore.currentRunning;
  const compressResult = await compressImage(getProfileActive(), file, parallelImages);
  if (compressResult.status === "error") {
    if (compressResult.error.errorType === "NotSmaller") {
      updateFile(file, {
        error: compressResult.error.error,
        status: "AlreadySmaller",
      });
      return;
    }
    updateFile(file, { error: compressResult.error.error, status: "Error" });
    return;
  }

  const outSize = compressResult.data.outSize;
  let savings = null;
  if (file.originalSize !== null) {
    savings = ((file.originalSize - outSize) / file.originalSize) * 100;
  }
  file = updateFile(file, {
    status: "Complete",
    size: outSize,
    savings,
  });
}

function updateFile(
  file: FileEntry,
  update: Partial<FileEntry>,
): ReadonlyFileEntry {
  const newFile: ReadonlyFileEntry = { ...file, ...update };
  setStore("files", (f) => f.path === file.path, newFile);
  return newFile;
}

function isFileDone(file: ReadonlyFileEntry): boolean {
  return (
    file.status === "Complete" ||
    file.status === "AlreadySmaller" ||
    file.status === "Error"
  );
}

function updateDockBadge() {
  const remaining = store.files.filter((f) => !isFileDone(f)).length;
  commands.setDockBadge(remaining);

  // Bounce dock icon when all files are done
  if (remaining === 0 && store.files.length > 0) {
    commands.bounceDockIcon();
  }
}

function clearFiles() {
  semaphore.cancel();
  setStore("files", []);
  commands.setDockBadge(0);
}

function removeFile(file: FileEntry) {
  setStore("files", (f) => f.filter((f) => f.path !== file.path));
}

export { store, addFile, updateFile, clearFiles, removeFile };
