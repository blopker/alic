import { createSignal } from "solid-js";
import type { UpdateStateEvent } from "./bindings";
import { addToast } from "./Toast";

const [updateState, setUpdateState] = createSignal<UpdateStateEvent | null>();

export function showUpdateToast(result: UpdateStateEvent) {
  if (result.type === "CheckingForUpdate") {
    addToast({
      message: <UpdateNotification />,
      type: "info",
      duration: -1,
    });
  }
  setUpdateState(result);
}

function UpdateNotification() {
  const updateTextMap = () => {
    const state = updateState();
    if (!state) {
      return "Checking for updates...";
    }
    switch (state.type) {
      case "CheckingForUpdate":
        return "Checking for updates...";
      case "Downloading":
        return <Downloading percent={state.percent} />;
      case "NoUpdate":
        return "No updates available.";
      case "Error":
        return <span>Update error: {state.message}</span>;
      case "Success":
        return `Update ${state.version} installed. Restarting...`;
    }
  };
  return <span>{updateTextMap()}</span>;
}

function Downloading(props: { percent: number | null }) {
  return (
    <span>
      Downloading update: {props.percent ? props.percent.toFixed(2) : "?"}%
    </span>
  );
}
