// scripts/release.ts
import fs from "node:fs";
import { $ } from "bun";
import * as readline from "node:readline";

const tauriConfPath = "src-tauri/tauri.conf.json";
const cargoTomlPath = "src-tauri/Cargo.toml";
const cargoLockPath = "src-tauri/Cargo.lock";

function getVersion() {
  // Read current version from tauri.conf.json
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
  return tauriConf.version;
}

function bumpPatchVersion(version: string): string {
  const parts = version.split(".");
  const patch = parseInt(parts[2], 10) + 1;
  return `${parts[0]}.${parts[1]}.${patch}`;
}

function setVersion(version: string) {
  // Update tauri.conf.json
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
  tauriConf.version = version;
  fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + "\n");

  // Update Cargo.toml
  let cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  cargoToml = cargoToml.replace(/^version = ".*"$/m, `version = "${version}"`);
  fs.writeFileSync(cargoTomlPath, cargoToml);
}

async function prompt(question: string): Promise<string> {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });
  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      rl.close();
      resolve(answer.trim());
    });
  });
}

async function main() {
  try {
    // Make sure we're on main branch and it's clean
    await $`git diff-index --quiet HEAD --`;
    await $`git checkout main`;
    await $`git pull origin main`;

    // Bump patch version with user confirmation
    const currentVersion = getVersion();
    const suggestedVersion = bumpPatchVersion(currentVersion);
    const input = await prompt(
      `Current version: ${currentVersion}\nEnter new version (press Enter for ${suggestedVersion}): `,
    );
    const version = input || suggestedVersion;

    if (!/^\d+\.\d+\.\d+$/.test(version)) {
      console.error("Invalid version format. Expected x.y.z");
      process.exit(1);
    }

    setVersion(version);
    console.log(`Version: ${currentVersion} -> ${version}`);

    // Update Cargo.lock by running cargo check
    await $`cargo check --manifest-path ${cargoTomlPath}`.quiet();

    // Commit the version bump
    await $`git add ${tauriConfPath} ${cargoTomlPath} ${cargoLockPath}`;
    await $`git commit -m "v${version}"`;

    // Push version bump to main
    await $`git push origin main`;

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
