import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
import { onMount } from "solid-js";
import BottomBar from "./BottomBar";
import Dropper from "./Dropper";
import { errorListener, updateResultListener } from "./listeners";
import ProgressBar from "./ProgressBar";
import { addFile } from "./store";
import Table from "./Table";
import { addToast, ToastContainer } from "./Toast";
import { showUpdateToast } from "./updater";

updateResultListener(showUpdateToast);
errorListener((message) => {
  addToast({
    message,
    type: "error",
  });
});

function handleDeepLink(urls: string[]) {
  // [Log] deep link: – ["file:///Users/myuser/Downloads/file.jpg"]
  // console.log("deep link:", urls);
  for (const url of urls) {
    if (url.startsWith("file://")) {
      // decodeURI misses reserved characters like %23 (#), so parse properly
      addFile(decodeURIComponent(new URL(url).pathname));
    } else {
      addFile(decodeURI(url));
    }
  }
}

function App() {
  // Deep links are handled inside the component, not at module scope: this
  // module also loads in the settings window, which must not add files.
  // App only mounts in the main window.
  onMount(async () => {
    handleDeepLink((await getCurrent()) ?? []);
    await onOpenUrl(handleDeepLink);
  });
  return (
    <div class="flex h-screen select-none flex-col">
      <ToastContainer />
      <Dropper />
      <main class="w-full grow overflow-hidden">
        <Table />
      </main>
      <ProgressBar />
      <BottomBar />
    </div>
  );
}

export default App;
