export interface FileResult {
  path: string;
  name: string;
  extension: string | null;
  kind: string;
  size: number;
}

export interface SearchOutput {
  results: FileResult[];
  total: number;
  returned: number;
  duration_ms: number;
}
