import type {
  FileEntry,
  ProfileData_Serialize as ProfileData,
} from "./bindings";
import { commands } from "./bindings";

export async function compressImage(
  profile: ProfileData,
  file: FileEntry,
  parallelImages: number,
) {
  return await commands.processImg(profile, file, parallelImages);
}
