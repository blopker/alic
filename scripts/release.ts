// scripts/release.ts
import { execSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";

function updateChangelog(newVersion: string): void {
  const date = new Date().toISOString().split("T")[0];
  const changelogPath = "CHANGELOG.md";
  const changelog = fs.readFileSync(changelogPath, "utf8");

  // Split changelog into lines
  const lines = changelog.split("\n");

  // Find the Unreleased section
  const unreleasedIndex = lines.findIndex((line) =>
    line.includes("## Unreleased"),
  );
  if (unreleasedIndex === -1) {
    throw new Error("Could not find Unreleased section in CHANGELOG.md");
  }

  // Find the next version section
  const nextVersionIndex = lines.findIndex(
    (line, i) => i > unreleasedIndex && line.startsWith("## ["),
  );

  // Get unreleased changes
  const unreleasedChanges = lines
    .slice(
      unreleasedIndex + 1,
      nextVersionIndex === -1 ? undefined : nextVersionIndex,
    )
    .filter((line) => line.trim() !== "");

  // Create new version section with unreleased changes
  const newVersionSection = [
    `## [${newVersion}] - ${date}\n`,
    ...unreleasedChanges,
    "\n",
  ].join("\n");

  // Reset unreleased section
  const newChangelog = [
    "# Changelog",
    "",
    "## Unreleased",
    "",
    newVersionSection,
    ...lines.slice(nextVersionIndex === -1 ? lines.length : nextVersionIndex),
  ].join("\n");

  fs.writeFileSync(changelogPath, newChangelog);
}

function updateVersion(type: "major" | "minor" | "patch") {
  // Read current version from tauri.conf.json
  const tauriConfPath = "src-tauri/tauri.conf.json";
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, "utf8"));
  const versionMatch = tauriConf.version.match(/(\d+)\.(\d+)\.(\d+)/);

  if (!versionMatch) {
    throw new Error("Could not find version in tauri.conf.json");
  }

  let [, major, minor, patch] = versionMatch.map(Number);

  // Update version numbers
  switch (type) {
    case "major":
      major++;
      minor = 0;
      patch = 0;
      break;
    case "minor":
      minor++;
      patch = 0;
      break;
    case "patch":
      patch++;
      break;
  }

  const newVersion = `${major}.${minor}.${patch}`;
  tauriConf.version = newVersion;
  fs.writeFileSync(tauriConfPath, `${JSON.stringify(tauriConf, null, 2)}\n`);
  return newVersion;
}

function main() {
  const type = Bun.argv[2] as "major" | "minor" | "patch";
  if (!["major", "minor", "patch"].includes(type)) {
    console.error("Usage: bun run release.ts <major|minor|patch>");
    process.exit(1);
  }

  try {
    // Make sure we're on main branch and it's clean
    execSync("git diff-index --quiet HEAD --");
    execSync("git checkout main");
    execSync("git pull origin main");

    // Update version
    const newVersion = updateVersion(type);

    // Create changelog entry
    updateChangelog(newVersion);

    // Commit and push changes
    execSync(`git commit -am "Bump version to ${newVersion}"`);
    // Delete existing release branch locally and remotely if it exists
    try {
      execSync("git branch -D release");
    } catch (e) {
      // Branch doesn't exist locally, that's fine
    }
    try {
      execSync("git push origin :release");
    } catch (e) {
      // Branch doesn't exist remotely, that's fine
    }

    // Create and push new release branch
    execSync("git checkout -b release");
    execSync("git push origin release --force");
    execSync(`git tag -a v${newVersion} -m "Release v${newVersion}"`);
    execSync("git push origin --tags");

    // Go back to main
    execSync("git checkout main");

    console.log(`\nReleased version ${newVersion}!`);
    console.log("1. Update CHANGELOG.md with the actual changes");
    console.log("2. Wait for GitHub Actions to finish");
    console.log("3. Go to GitHub releases to review and publish");
  } catch (error) {
    console.error("Error:", error);
    process.exit(1);
  }
}

main();
