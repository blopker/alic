import { addToast } from "./Toast";
import { commands } from "./bindings";

export function showUpdateToast(result: string) {
  addToast({
    message: <UpdateNotification result={result} />,
    type: "info",
    duration: -1,
  });
}

function UpdateNotification(props: { result: string }) {
  return (
    <span>
      {props.result}{" "}
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
  );
}
