import { onOpenUrl } from "@tauri-apps/plugin-deep-link";
import BottomBar from "./BottomBar";
import Dropper from "./Dropper";
import Table from "./Table";
import { ToastContainer, addToast } from "./Toast";
import { commands } from "./bindings";
import { updateResultListener } from "./listeners";
import { addFile } from "./store";

updateResultListener((result) => {
  addToast({
    message: (
      <span>
        {result}{" "}
        <button
          class="text-blue-500"
          type="button"
          onClick={() =>
            commands.openLinkInBrowser(
              "https://github.com/blopker/alic/releases/",
            )
          }
        >
          See Releases.
        </button>
      </span>
    ),
    type: "info",
    duration: -1,
  });
});

onOpenUrl((urls) => {
  console.log("deep link:", urls);
  // [Log] deep link: – ["file:///Users/myuser/Downloads/file.jpg"]
  for (const url of urls) {
    addFile(decodeURI(url.replace("file://", "")));
  }
});

function App() {
  return (
    <div class="flex h-screen select-none flex-col">
      <ToastContainer />
      <Dropper />
      <main class="w-full grow overflow-scroll">
        <Table />
      </main>
      <BottomBar />
    </div>
  );
}

export default App;
