import { createMemo, Show } from "solid-js";
import { store } from "./store";

export default function ProgressBar() {
  const totalFiles = () => store.files.length;
  const completedFiles = () =>
    store.files.filter(
      (f) =>
        f.status === "Complete" ||
        f.status === "AlreadySmaller" ||
        f.status === "Error",
    ).length;

  const progress = createMemo(() => {
    if (totalFiles() === 0) return 0;
    return (completedFiles() / totalFiles()) * 100;
  });

  const isProcessing = () =>
    totalFiles() > 0 && completedFiles() < totalFiles();

  return (
    <Show when={isProcessing()}>
      <div class="h-1 w-full bg-secondary">
        <div
          class="h-full transition-all duration-300 ease-out"
          style={{
            width: `${progress()}%`,
            "background-color": "var(--input-accent-color)",
          }}
        />
      </div>
    </Show>
  );
}
