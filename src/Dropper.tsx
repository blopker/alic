import type { Event } from "@tauri-apps/api/event";
import { type DragDropEvent, getCurrentWebview } from "@tauri-apps/api/webview";
import { BsArrowDownSquare } from "solid-icons/bs";
import { Show, createSignal, onCleanup } from "solid-js";
import { Transition } from "solid-transition-group";
import { commands } from "./bindings";
import { addFileListener } from "./listeners";
import { addFile } from "./store";

addFileListener((path) => {
  addFile(path);
});

export default function Dropper() {
  const [showDropper, setShowDropper] = createSignal(false);
  const cancel = getCurrentWebview().onDragDropEvent(
    async (e: Event<DragDropEvent>) => {
      if (e.payload.type === "enter") {
        // if (anyImage(...e.payload.paths)) {
        setShowDropper(true);
        // }
      } else if (e.payload.type === "leave") {
        setShowDropper(false);
      } else if (e.payload.type === "drop") {
        setShowDropper(false);
        for (const path of e.payload.paths) {
          commands.getAllImages(path);
        }
      }
    },
  );
  onCleanup(() => {
    cancel.then((cancel) => cancel());
  });
  return (
    <Transition name="fade">
      <Show when={showDropper()}>
        <DropOverlay />
      </Show>
    </Transition>
  );
}

function DropOverlay() {
  return (
    <div class="frost absolute top-0 left-0 z-50 flex h-full w-full items-center justify-center bg-primary/60 transition-all">
      <BsArrowDownSquare class="opacity-75" size={300} />
    </div>
  );
}
