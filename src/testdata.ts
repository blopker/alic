import { createStore } from "solid-js/store";
import type { FileEntry } from "./bindings";

const files: FileEntry[] = [
  {
    path: "test/test.png",
    file: "test.png",
    status: "Processing",
    size: 100,
    originalSize: 2,
    ext: "png",
    error: null,
    savings: null,
  },
  {
    path: "test/test.png",
    file: "test.png",
    status: "Compressing",
    size: null,
    originalSize: null,
    ext: "png",
    error: null,
    savings: 100,
  },
  {
    path: "test/test.png",
    file: "test.png",
    status: "Complete",
    size: null,
    originalSize: null,
    ext: "png",
    error: null,
    savings: 100,
  },
  {
    path: "test/test.png",
    file: "test.png",
    status: "Error",
    size: null,
    originalSize: null,
    ext: "png",
    error: "Ruhoh",
    savings: 100,
  },
];

const [testStore, setStore] = createStore({
  files: files,
});

function updateFile(file: FileEntry) {
  setStore("files", (f) => f.path === file.path, file);
}

// setInterval(() => {
//   for (const file of testStore.files) {
//     // randomize status
//     if (Math.random() < 0.3) {
//       updateFile({ ...file, status: "Processing" });
//     } else if (Math.random() < 0.5) {
//       updateFile({ ...file, status: "Compressing" });
//     } else if (Math.random() < 0.8) {
//       updateFile({ ...file, status: "Complete" });
//     } else {
//       updateFile({ ...file, status: "Error" });
//     }
//   }
// }, 1000);

export { testStore, updateFile };
