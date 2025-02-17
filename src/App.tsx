import { getCurrent, onOpenUrl } from "@tauri-apps/plugin-deep-link";
import BottomBar from "./BottomBar";
import Dropper from "./Dropper";
import Table from "./Table";
import { ToastContainer } from "./Toast";
import { addToast } from "./Toast";
import { badFileListener, updateResultListener } from "./listeners";
import { addFile } from "./store";
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
    addFile(decodeURI(url.replace("file://", "")));
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
      <BottomBar />
    </div>
  );
}

export default App;
