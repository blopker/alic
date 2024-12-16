import { commands } from "./bindings";
import type { FileEntry, ProfileData } from "./bindings";

export async function compressImage(profile: ProfileData, file: FileEntry) {
  return await commands.processImg(profile, file);
}
