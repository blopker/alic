import Tooltip from "@corvu/tooltip";
import { BsArrowDown, BsArrowDownSquare, BsArrowUp } from "solid-icons/bs";
import { FaSolidCircleNotch, FaSolidMinus, FaSolidXmark } from "solid-icons/fa";
import { FaSolidCheck } from "solid-icons/fa";
import { TbDots } from "solid-icons/tb";
import { For, type JSXElement, Match, Switch, splitProps } from "solid-js";
import { Show, createSignal } from "solid-js";
import type { FileEntry, FileEntryStatus } from "./bindings";
import { commands } from "./bindings";
import { store } from "./store";
import { testStore } from "./testdata";
import { toHumanReadableSize } from "./utils";

const useTestData = false;
// const useTestData = true;

const statusOrder: Array<FileEntryStatus> = [
  "Compressing",
  "Processing",
  "Complete",
  "AlreadySmaller",
  "Error",
];

function StatusIcons(props: { status: FileEntryStatus }) {
  return (
    <Switch>
      <Match when={props.status === "Processing"}>
        <TbDots />
      </Match>
      <Match when={props.status === "Compressing"}>
        <FaSolidCircleNotch class="animate-spin" />
      </Match>
      <Match when={props.status === "Complete"}>
        <FaSolidCheck />
      </Match>
      <Match when={props.status === "Error"}>
        <FaSolidXmark />
      </Match>
      <Match when={props.status === "AlreadySmaller"}>
        <FaSolidMinus />
      </Match>
    </Switch>
  );
}

function MyTable() {
  // Table with columns of status, file, savings, size
  const [sortField, setSortField] = createSignal<keyof FileEntry | null>();
  const [sortDirection, setSortDirection] = createSignal<"asc" | "desc">("asc");
  const SortIcon = (props: { field: string }) => (
    <span>
      <Switch>
        <Match when={sortField() === props.field}>
          <Show when={sortDirection() === "asc"}>
            <BsArrowUp />
          </Show>
          <Show when={sortDirection() === "desc"}>
            <BsArrowDown />
          </Show>
        </Match>
      </Switch>
    </span>
  );
  const MyTH = (props: {
    field: keyof FileEntry;
    children: JSXElement;
    class?: string;
  }) => (
    //biome-ignore lint/a11y/useKeyWithClickEvents: <explanation>
    <th
      colspan={1}
      onClick={() => handleSort(props.field)}
      class={`px-2 text-left capitalize ${props.class ?? ""}`}
    >
      <div class="flex cursor-pointer items-center whitespace-nowrap align-middle">
        {props.children}
        <SortIcon field={props.field} />
      </div>
    </th>
  );

  const MyTD = (props: { children: JSXElement; class?: string }) => {
    const [local, others] = splitProps(props, ["children"]);
    return (
      <td {...others} class={`px-2 ${props.class}`}>
        {local.children}
      </td>
    );
  };
  const handleSort = (field: keyof FileEntry) => {
    if (sortField() === field) {
      setSortDirection((prev) => (prev === "asc" ? "desc" : "asc"));
    } else {
      setSortField(field);
      setSortDirection("asc");
    }
  };
  const sortedFiles = () => {
    const field = sortField();
    let files = [...store.files];
    if (useTestData) {
      // display files 50 times
      files = [...Array(50)].map(
        () =>
          testStore.files[Math.floor(Math.random() * testStore.files.length)],
      );
    }
    if (!field) {
      return files;
    }
    return files.sort((a, b) => {
      let aValue = a[field] ?? "";
      let bValue = b[field] ?? "";
      if (field === "status") {
        aValue = statusOrder.indexOf(a.status);
        bValue = statusOrder.indexOf(b.status);
      }
      if (aValue < bValue) return sortDirection() === "asc" ? -1 : 1;
      if (aValue > bValue) return sortDirection() === "asc" ? 1 : -1;
      return 0;
    });
  };
  return (
    <div class="flex h-full w-full select-none flex-col">
      <table class="min-w-full">
        <thead class="sticky top-0 z-40 bg-secondary shadow-lg">
          <tr>
            <MyTH class="w-12" field="status">
              S
            </MyTH>
            <MyTH field="file">File</MyTH>
            <MyTH class="w-28" field="savings">
              Savings
            </MyTH>
            <MyTH class="w-24" field="originalSize">
              Size
            </MyTH>
          </tr>
        </thead>
      </table>
      <div class="grow overflow-y-auto">
        <table class="min-w-full">
          <tbody class="text-clip">
            <For each={sortedFiles()}>
              {(file) => (
                <tr
                  onDblClick={() => {
                    console.log(commands.openFinderAtPath(file.path));
                  }}
                  class="cursor-default even:bg-secondary hover:bg-accent"
                >
                  <Tooltip>
                    <Tooltip.Trigger as={MyTD} class="w-12">
                      <StatusIcons status={file.status} />
                    </Tooltip.Trigger>
                    <Tooltip.Portal>
                      <Tooltip.Content class="border-[1px] border-accent bg-secondary px-2 py-1">
                        <Show when={file.error} fallback={file.status}>
                          {file.error}
                        </Show>
                      </Tooltip.Content>
                    </Tooltip.Portal>
                  </Tooltip>
                  <Tooltip>
                    <Tooltip.Trigger as={MyTD}>{file.file}</Tooltip.Trigger>
                    <Tooltip.Portal>
                      <Tooltip.Content class="border-[1px] border-accent bg-secondary px-2 py-1">
                        {file.path}
                      </Tooltip.Content>
                    </Tooltip.Portal>
                  </Tooltip>
                  <MyTD class="w-28">
                    <Show when={file.savings} fallback="?">
                      {(file.savings ?? 0).toFixed(1)}%
                    </Show>
                  </MyTD>
                  <MyTD class="w-24">
                    <Show
                      when={file.size}
                      fallback={toHumanReadableSize(file.originalSize)}
                    >
                      {toHumanReadableSize(file.size)}
                    </Show>
                  </MyTD>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </div>
    </div>
  );
}

function PlaceHolder() {
  return (
    <div class="flex h-full w-full grow items-center justify-center">
      <BsArrowDownSquare size={100} class="opacity-50" />
    </div>
  );
}

function TableOrPlaceholder() {
  return (
    <Show
      when={store.files.length > 0 || useTestData}
      fallback={<PlaceHolder />}
    >
      <MyTable />
    </Show>
  );
}

export default TableOrPlaceholder;
