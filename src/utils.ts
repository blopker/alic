function toHumanReadableSize(size?: number | null) {
  if (!size) {
    return "?";
  }
  if (size < 1024) {
    return `${size} B`;
  }
  const units = ["B", "kB", "MB", "GB", "TB"];
  const i = Math.min(
    Math.floor(Math.log(size) / Math.log(1024)),
    units.length - 1,
  );
  return `${(size / 1024 ** i).toFixed(1)} ${units[i]}`;
}

class Semaphore {
  private running = 0;
  private queue: (() => void)[] = [];

  constructor(public maxConcurrent: number) {}

  get currentRunning(): number {
    return this.running;
  }

  async acquire(): Promise<void> {
    if (this.running >= this.maxConcurrent) {
      return new Promise<void>((resolve) => {
        this.queue.push(resolve);
      });
    }
    this.running++;
  }

  cancel(): void {
    // Wake all waiters so their callers can run cleanup; count them as
    // running so the release() in each caller's finally stays balanced.
    const waiters = this.queue;
    this.queue = [];
    for (const waiter of waiters) {
      this.running++;
      waiter();
    }
  }

  release(): void {
    this.running--;
    if (this.running < 0) {
      this.running = 0;
    }
    while (this.running < this.maxConcurrent) {
      const next = this.queue.shift();
      if (next) {
        this.running++;
        next();
      } else {
        break;
      }
    }
  }
}

export { toHumanReadableSize, Semaphore };
