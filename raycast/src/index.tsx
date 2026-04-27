import { Action, ActionPanel, Icon, List } from "@raycast/api";
import { useState, useEffect } from "react";
import { searchFiles, isBinaryAvailable } from "./cli";
import type { FileResult } from "./types";

function humansize(bytes: number): string {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;
  if (bytes >= GB) return `${(bytes / GB).toFixed(1)} GB`;
  if (bytes >= MB) return `${(bytes / MB).toFixed(1)} MB`;
  if (bytes >= KB) return `${(bytes / KB).toFixed(1)} KB`;
  return `${bytes} B`;
}

function fileIcon(kind: string, extension: string | null): Icon {
  if (kind === "directory") return Icon.Folder;
  const ext = extension?.toLowerCase();
  switch (ext) {
    case "pdf":
      return Icon.Document;
    case "png":
    case "jpg":
    case "jpeg":
    case "gif":
    case "svg":
    case "webp":
      return Icon.Image;
    case "zip":
    case "tar":
    case "gz":
    case "rar":
    case "7z":
      return Icon.Box;
    case "js":
    case "ts":
    case "tsx":
    case "jsx":
    case "py":
    case "rs":
    case "go":
    case "java":
    case "swift":
      return Icon.Code;
    default:
      return Icon.Document;
  }
}

export default function Command() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<FileResult[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!query.trim()) {
      setResults([]);
      setError(null);
      return;
    }

    setIsLoading(true);
    setError(null);

    const timer = setTimeout(() => {
      try {
        if (!isBinaryAvailable()) {
          setError("cardinal-cli not found. Install it or set the path in preferences.");
          setResults([]);
          return;
        }

        const output = searchFiles(query);
        setResults(output.results);
      } catch (err) {
        setError(err instanceof Error ? err.message : "Search failed");
        setResults([]);
      } finally {
        setIsLoading(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [query]);

  return (
    <List
      searchBarPlaceholder="Search files (Everything-compatible syntax)…"
      onSearchTextChange={setQuery}
      isLoading={isLoading}
      throttle
    >
      {error ? (
        <List.EmptyView title="Error" description={error} icon={Icon.ExclamationMark} />
      ) : results.length === 0 && query.trim() ? (
        <List.EmptyView title="No Results" description={`No files matching "${query}"`} icon={Icon.MagnifyingGlass} />
      ) : results.length === 0 ? (
        <List.EmptyView title="Cardinal Search" description="Type to search your files" icon={Icon.MagnifyingGlass} />
      ) : (
        results.map((file, index) => (
          <List.Item
            key={file.path}
            title={file.name}
            subtitle={file.path}
            icon={fileIcon(file.kind, file.extension)}
            accessories={[
              { text: humansize(file.size) },
              { text: file.extension?.toUpperCase() || file.kind },
            ]}
            actions={
              <ActionPanel>
                <Action.Open title="Open" target={file.path} />
                <Action.ShowInFinder title="Reveal in Finder" path={file.path} />
                <Action.CopyToClipboard
                  title="Copy Path"
                  content={file.path}
                  shortcut={{ modifiers: ["cmd", "shift"], key: "c" }}
                />
              </ActionPanel>
            }
          />
        ))
      )}
    </List>
  );
}
