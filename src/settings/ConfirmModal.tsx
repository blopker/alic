import { useKeyDownEvent } from "@solid-primitives/keyboard";
import { createEffect, createSignal, Show, untrack } from "solid-js";

import { Portal } from "solid-js/web";
import { SettingsButton } from "./SettingsUI";

interface ConfirmOpts {
  onCancel: () => void;
  onConfirm: () => void;
  text: string;
  show: boolean;
}

function defaultConfirmOpts(): ConfirmOpts {
  return {
    onCancel: () => {},
    onConfirm: () => {},
    // text: "Are you sure you want to reset all settings?",
    // show: true,
    text: "",
    show: false,
  };
}

const [confirmOpts, setConfirmOpts] =
  createSignal<ConfirmOpts>(defaultConfirmOpts());

function ConfirmModal() {
  const event = useKeyDownEvent();
  createEffect(() => {
    const e = event();
    untrack(() => {
      if (e && e.key === "Escape") {
        confirmOpts().onCancel();
        setConfirmOpts(defaultConfirmOpts());
      }
    });
  });
  return (
    <Show when={confirmOpts().show}>
      <Portal>
        <div class="absolute top-0 left-0 flex h-screen w-screen items-center justify-center bg-black/80 align-middle">
          <div class="flex h-40 w-80 flex-col rounded-md bg-primary">
            <div class="flex grow items-center justify-center align-middle">
              <div class="p-5 text-center text-lg">{confirmOpts().text}</div>
            </div>
            <div class="flex gap-2 p-4 pt-0">
              <SettingsButton
                style="secondary"
                autoFocus={true}
                onClick={() => {
                  confirmOpts().onCancel();
                  setConfirmOpts(defaultConfirmOpts());
                }}
              >
                Cancel
              </SettingsButton>
              <SettingsButton
                onClick={() => {
                  confirmOpts().onConfirm();
                  setConfirmOpts(defaultConfirmOpts());
                }}
              >
                OK
              </SettingsButton>
            </div>
          </div>
        </div>
      </Portal>
    </Show>
  );
}

function confirmModal(opts: Partial<ConfirmOpts>) {
  setConfirmOpts({
    ...defaultConfirmOpts(),
    ...opts,
    show: true,
  });
}

export { ConfirmModal, confirmModal };
