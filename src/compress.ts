import type { FileEntry, ProfileData } from "./bindings";
import { commands } from "./bindings";

export async function compressImage(profile: ProfileData, file: FileEntry) {
  return await commands.processImg(profile, file);
}
