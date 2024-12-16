import { execSync } from "node:child_process";

async function getCommitsSinceLastTag(): Promise<string> {
  try {
    const lastTag = execSync("git describe --tags --abbrev=0")
      .toString()
      .trim();
    const commits = execSync(
      `git log ${lastTag}..HEAD --pretty=format:"%s"`,
    ).toString();
    return commits;
  } catch (error) {
    // If no tags exist, get all commits
    return execSync('git log --pretty=format:"%s"').toString();
  }
}

async function generateChangelog() {
  const commits = await getCommitsSinceLastTag();

  const prompt = `
Given these git commits, generate a changelog in markdown format.
Group similar changes together under appropriate headers like "Features", "Bug Fixes", "Improvements", etc.
Make it clear and user-friendly.

Commits:
${commits}
`;

  try {
    const response =
      execSync(`curl -X POST http://localhost:11434/api/generate -d '{
      "model": "mistral-nemo",
      "prompt": ${JSON.stringify(prompt)}
    }'`).toString();

    // Parse the response
    const lines = response
      .split("\n")
      .filter((line) => line.trim())
      .map((line) => JSON.parse(line));

    // Combine all response parts
    const changelog = lines.map((line) => line.response).join("");

    console.log("\nGenerated Changelog:\n");
    console.log(changelog);
  } catch (error) {
    console.error("Error: Make sure Ollama is running with the mistral model");
    console.error("Run: ollama run mistral");
    process.exit(1);
  }
}

generateChangelog();
