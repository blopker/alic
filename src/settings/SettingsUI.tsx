import Tooltip from "@corvu/tooltip";

import { IoHelpCircleOutline } from "solid-icons/io";
import { For, type JSXElement, Show, onMount } from "solid-js";

function SettingsButton(props: {
  disabled?: boolean;
  onClick: () => void;
  children: JSXElement;
  style?: "secondary" | "primary" | "danger";
  autoFocus?: boolean;
}) {
  return (
    <button
      autofocus={props.autoFocus === true}
      disabled={props.disabled === true}
      onClick={props.onClick}
      type="button"
      classList={{
        "bg-indigo-600 text-white": [undefined, "primary"].includes(
          props.style,
        ),
        "bg-accent": props.style === "secondary",
        "bg-red-500 hover:bg-red-700": props.style === "danger",
      }}
      class="col-start-2 inline-flex w-full justify-center rounded-md px-3 py-2 font-semibold text-sm shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-indigo-600 focus-visible:outline-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
    >
      {props.children}
    </button>
  );
}

function SettingBox(props: { title: string; children: JSXElement }) {
  return (
    <div>
      <div class="pb-2">{props.title}</div>
      <div class="flex flex-col gap-4 rounded-xl border-2 border-accent p-4">
        {props.children}
      </div>
    </div>
  );
}

function SettingRow(props: {
  title: string;
  helpText?: string;
  children: JSXElement;
}) {
  return (
    <div class="flex items-center justify-between gap-2">
      <div class="flex items-center gap-1">
        {props.title}
        <Show when={props.helpText}>
          <Tooltip>
            <Tooltip.Trigger
              as={IoHelpCircleOutline}
              size={17}
              class="inline cursor-help opacity-50"
            />
            <Tooltip.Portal>
              <Tooltip.Content class="max-w-[30rem] border-[1px] border-accent bg-secondary px-2 py-1">
                {props.helpText}
              </Tooltip.Content>
            </Tooltip.Portal>
          </Tooltip>
        </Show>
      </div>
      <div>{props.children}</div>
    </div>
  );
}

function SettingsToggle(props: {
  value: boolean;
  onChange: (value: boolean) => void;
}) {
  return (
    <button
      type="button"
      class="relative inline-flex h-6 w-11 shrink-0 rounded-full border-2 border-transparent bg-gray-200 transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
      role="switch"
      aria-checked={props.value}
      classList={{
        "bg-indigo-600": props.value === true,
        "bg-gray-200": props.value === false,
      }}
      onClick={() => {
        props.onChange(!props.value);
      }}
    >
      <span
        aria-hidden="true"
        class="pointer-events-none inline-block size-5 translate-x-0 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
        classList={{
          "translate-x-5": props.value === true,
        }}
      />
    </button>
  );
}

function SettingsSelect(props: {
  value: string;
  onChange: (value: string) => void;
  options: string[];
  bgColor?: "bg-accent" | "bg-primary";
  class?: string;
}) {
  return (
    <select
      class={`${props.class} rounded-md border-0 py-1.5 shadow-sm sm:text-sm/6`}
      classList={{
        "bg-secondary": !props.bgColor,
        "bg-accent": props.bgColor === "bg-accent",
        "bg-primary": props.bgColor === "bg-primary",
      }}
      value={props.value}
      onChange={(e) => {
        props.onChange(e.target.value);
      }}
    >
      <For each={props.options}>
        {(option) => (
          <option selected={option === props.value}>{option}</option>
        )}
      </For>
    </select>
  );
}

function SettingsInput(props: {
  label: string;
  value: string;
  class?: string;
  placeholder?: string;
  autoFocus?: boolean;
  maxLength?: number;
  onChange: (value: string) => void;
}) {
  let inputRef: HTMLInputElement | undefined;
  onMount(() => {
    if (inputRef && props.autoFocus) {
      inputRef.focus();
    }
  });
  const maxLength = () => props.maxLength ?? 100;
  return (
    <input
      ref={inputRef}
      placeholder={props.placeholder}
      autofocus={props.autoFocus}
      class={`${props.class} rounded-md border-0 bg-secondary py-1.5 shadow-sm sm:text-sm/6`}
      type="text"
      maxLength={maxLength()}
      value={props.value}
      onInput={(e) => {
        props.onChange(e.target.value);
      }}
    />
  );
}

function SettingsNumberInput(props: {
  value: number;
  onChange: (value: number) => void;
}) {
  return (
    <input
      class="w-20 rounded-md border-0 bg-secondary py-1.5 shadow-sm sm:text-sm/6"
      type="text"
      min="1"
      value={props.value}
      onInput={(e) => {
        const value = Number.parseInt(e.target.value);
        if (Number.isNaN(value)) {
          return;
        }
        props.onChange(value);
      }}
    />
  );
}

function SettingsPage(props: { title: string; children: JSXElement }) {
  return (
    <div class="relative flex flex-col">
      <div class="flex h-[40px] items-center border-b border-b-accent pl-4">
        <h1 class="text-left font-bold text-lg">{props.title}</h1>
      </div>
      <div class="max-h-[calc(100vh-50px)] grow overflow-y-auto p-4">
        {props.children}
      </div>
    </div>
  );
}

export {
  SettingBox,
  SettingRow,
  SettingsToggle,
  SettingsSelect,
  SettingsInput,
  SettingsNumberInput,
  SettingsButton,
  SettingsPage,
};
