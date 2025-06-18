// types.ts
import { type Component, For, type JSXElement } from "solid-js";
import { createStore } from "solid-js/store";
import { TransitionGroup } from "solid-transition-group";
export type ToastType = "success" | "error" | "info" | "warning";

import { FaSolidXmark } from "solid-icons/fa";

export interface Toast {
  id: number;
  message: JSXElement;
  type: ToastType;
  duration?: number;
}

const [toasts, setToasts] = createStore<Toast[]>([]);
// const [toasts, setToasts] = createStore<Toast[]>([
//   {
//     id: 1,
//     message: "Hello World",
//     type: "success",
//   },
//   {
//     id: 2,
//     message: "This is an error",
//     type: "error",
//   },
//   {
//     id: 3,
//     message:
//       "This is a warning. This is a warning .This is a warning This is a warning This is a warning This is a warning This is a warning",
//     type: "warning",
//   },
//   {
//     id: 4,
//     message: "This is an info",
//     type: "info",
//   },
// ]);

export const addToast = (toast: Omit<Toast, "id">) => {
  // get max id + 1
  const id = toasts.reduce((max, t) => Math.max(max, t.id), 0) + 1;
  const duration = toast.duration || 5000;

  setToasts([...toasts, { ...toast, id }]);

  if (duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, duration);
  }
};

export const removeToast = (id: number) => {
  setToasts(toasts.filter((toast) => toast.id !== id));
};

const toastStyles = {
  success: "border-t-green-500/50",
  error: "border-t-red-500/50",
  info: "border-t-blue-500/50",
  warning: "border-t-yellow-500/50",
};

const Toast: Component<{ toast: Toast; onClose: (id: number) => void }> = (
  props,
) => {
  return (
    <div
      class={`${toastStyles[props.toast.type]} flex items-center justify-between rounded-md border-[1px] border-secondary bg-accent px-6 py-2 text-white shadow-lg`}
      role="alert"
    >
      <span>{props.toast.message}</span>
      <button
        type="button"
        onClick={() => props.onClose(props.toast.id)}
        class="ml-4 transition-opacity hover:opacity-75"
      >
        <FaSolidXmark />
      </button>
    </div>
  );
};

export const ToastContainer: Component = () => {
  return (
    <div class="fixed top-0 right-0 z-50 flex flex-col gap-2 p-4">
      <TransitionGroup name="fade">
        <For each={toasts}>
          {(toast) => <Toast toast={toast} onClose={removeToast} />}
        </For>
      </TransitionGroup>
    </div>
  );
};
