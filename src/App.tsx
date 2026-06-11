import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
import BottomBar from "./BottomBar";
import Dropper from "./Dropper";
import { badFileListener, updateResultListener } from "./listeners";
import ProgressBar from "./ProgressBar";
import { addFile } from "./store";
import Table from "./Table";
import { addToast, ToastContainer } from "./Toast";
import { showUpdateToast } from "./updater";

updateResultListener(showUpdateToast);
badFileListener((path) => {
  addToast({
    message: `Unsupported file: ${path}`,
    type: "error",
  });
});

// Initialize deep link handling
handleDeepLink((await getCurrent()) ?? []);
onOpenUrl(handleDeepLink);
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
