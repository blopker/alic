import Tooltip from "@corvu/tooltip";
import { BsArrowDown, BsArrowDownSquare, BsArrowUp } from "solid-icons/bs";
import {
  FaSolidCheck,
  FaSolidCircleNotch,
  FaSolidMinus,
  FaSolidXmark,
} from "solid-icons/fa";
import { TbOutlineDots } from "solid-icons/tb";
import {
  createSignal,
  For,
  type JSXElement,
  Match,
  onCleanup,
  onMount,
  Show,
  Switch,
  splitProps,
} from "solid-js";
import type { FileEntry, FileEntryStatus } from "./bindings";
import { commands } from "./bindings";
import { removeFile, store } from "./store";
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
        <TbOutlineDots />
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
  const [selectedFiles, setSelectedFiles] = createSignal<Set<FileEntry>>(
    new Set(),
  );

  // Handle keyboard events for selection and removal
  const handleKeyDown = (e: KeyboardEvent) => {
    // Select all with Cmd+A
    if (e.key === "a" && (e.metaKey || e.ctrlKey)) {
      e.preventDefault();
      const allPaths = sortedFiles().map((file) => file);
      setSelectedFiles(new Set(allPaths));
    }

    // Remove selected files with Backspace or Delete
    if (e.key === "Backspace" || e.key === "Delete") {
      e.preventDefault();
      if (selectedFiles().size > 0) {
        removeSelectedFiles();
      }
    }

    // Escape to clear selection
    if (e.key === "Escape") {
      setSelectedFiles(new Set<FileEntry>());
    }
  };

  // Remove selected files from the queue
  const removeSelectedFiles = () => {
    for (const file of selectedFiles()) {
      removeFile(file);
    }
    setSelectedFiles(new Set<FileEntry>());
  };

  // Toggle file selection
  const toggleFileSelection = (file: FileEntry, e: MouseEvent) => {
    e.preventDefault();
    const newSelection = new Set(selectedFiles());

    if (e.shiftKey && selectedFiles().size > 0) {
      // Range selection
      const fileList = sortedFiles();
      const lastSelectedIndex = fileList.findIndex((f) =>
        selectedFiles().has(f),
      );
      const currentIndex = fileList.findIndex((f) => f.path === file.path);

      if (lastSelectedIndex !== -1) {
        const start = Math.min(lastSelectedIndex, currentIndex);
        const end = Math.max(lastSelectedIndex, currentIndex);
        for (let i = start; i <= end; i++) {
          newSelection.add(fileList[i]);
        }
      }
    } else if (e.metaKey || e.ctrlKey) {
      // Toggle selection for this file only
      if (newSelection.has(file)) {
        newSelection.delete(file);
      } else {
        newSelection.add(file);
      }
    } else {
      // Select only this file
      newSelection.clear();
      newSelection.add(file);
    }

    setSelectedFiles(newSelection);
  };

  onMount(() => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onCleanup(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

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
      <div
        class="grow overflow-y-auto"
        role="menu"
        onKeyPress={() => {}}
        onClick={(e) => {
          // Clear selection when clicking on empty space
          // Check if the click was directly on the container, not on a table row
          if (e.target === e.currentTarget) {
            setSelectedFiles(new Set<FileEntry>());
          }
        }}
      >
        <table class="min-w-full">
          <tbody class="text-clip">
            <For each={sortedFiles()}>
              {(file) => (
                <tr
                  onClick={(e) => toggleFileSelection(file, e)}
                  onKeyPress={() => {}}
                  onDblClick={() => {
                    console.log(commands.openFinderAtPath(file.path));
                  }}
                  class={`cursor-default even:bg-secondary hover:bg-accent ${
                    selectedFiles().has(file)
                      ? "!text-white !bg-(--input-accent-color)"
                      : ""
                  }`}
                >
                  <Tooltip>
                    <Tooltip.Trigger as={MyTD} class="w-12 cursor-help">
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
      <BsArrowDownSquare size={150} class="opacity-10" />
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
