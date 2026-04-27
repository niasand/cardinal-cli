import { execFileSync } from "child_process";
import { getPreferenceValues } from "@raycast/api";
import type { SearchOutput } from "./types";

interface Preferences {
  binaryPath: string;
  searchRoot: string;
  resultLimit: string;
  caseSensitive: boolean;
}

const DEFAULT_BINARY = "cardinal-cli";

function getBinaryPath(): string {
  const prefs = getPreferenceValues<Preferences>();
  return prefs.binaryPath?.trim() || DEFAULT_BINARY;
}

export function searchFiles(query: string): SearchOutput {
  const binary = getBinaryPath();
  const prefs = getPreferenceValues<Preferences>();

  const args = [
    "search",
    query,
    "--format", "json",
    "--limit", prefs.resultLimit || "50",
    "--path", prefs.searchRoot || "/",
  ];

  if (prefs.caseSensitive) {
    args.push("--case-sensitive");
  }

  const stdout = execFileSync(binary, args, {
    maxBuffer: 10 * 1024 * 1024,
    timeout: 30000,
    encoding: "utf-8",
  });

  return JSON.parse(stdout) as SearchOutput;
}

export function isBinaryAvailable(): boolean {
  try {
    execFileSync(getBinaryPath(), ["--version"], { timeout: 5000 });
    return true;
  } catch {
    return false;
  }
}
