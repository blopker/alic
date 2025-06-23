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

    // Get version
    const version = getVersion();

    // Delete existing release branch if it exists
    try {
      await $`git branch -D release`.quiet();
    } catch {
      // Branch doesn't exist, that's fine
    }

    // Create new release branch from main
    await $`git checkout -b release`;

    // Push to release branch (force push to overwrite any existing remote branch)
    await $`git push origin release --force`;

    // Switch back to main
    await $`git checkout main`;

    console.log(`\nReleased version ${version} to release branch!`);
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
