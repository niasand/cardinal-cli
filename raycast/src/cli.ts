import { execFileSync } from "child_process";
import type { SearchOutput } from "./types";

const BINARY_PATH = "/Users/bytedance/bin/cardinal-cli";
const SEARCH_ROOT = "/";
const RESULT_LIMIT = "50";

export function searchFiles(query: string): SearchOutput {
  const args = [
    "search",
    query,
    "--format", "json",
    "--limit", RESULT_LIMIT,
    "--path", SEARCH_ROOT,
  ];

  const stdout = execFileSync(BINARY_PATH, args, {
    maxBuffer: 10 * 1024 * 1024,
    timeout: 30000,
    encoding: "utf-8",
  });

  return JSON.parse(stdout) as SearchOutput;
}

export function isBinaryAvailable(): boolean {
  try {
    execFileSync(BINARY_PATH, ["--version"], { timeout: 5000 });
    return true;
  } catch {
    return false;
  }
}
