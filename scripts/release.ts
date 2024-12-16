// scripts/release.ts
import fs from "node:fs";
import { $ } from "bun";

function getVersion() {
  // Read current version from tauri.conf.json
  const tauriConfPath = "src-tauri/tauri.conf.json";
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
  return tauriConf.version;
}

async function main() {
  try {
    // Make sure we're on main branch and it's clean
    await $`git diff-index --quiet HEAD --`;
    await $`git checkout main`;
    await $`git pull origin main`;

    // Update version
    const newVersion = getVersion();
    const tag = `v${newVersion}`;

    // Check that the tag doesn't already exist
    const tags = await $`git tag --list`;
    if (tags.text().includes(tag)) {
      throw `Tag ${tag} already exists`;
    }

    await $`git tag -a v${newVersion} -m "Release v${newVersion}"`;
    await $`git push origin --tags --all`;

    console.log("\nReleasing version $newVersion!");
    console.log("1. Wait for GitHub Actions to finish");
    console.log(
      "2. Go to https://github.com/blopker/alic/releases to review and publish",
    );
  } catch (error) {
    console.error("Error:", error);
    process.exit(1);
  }
}

main();
