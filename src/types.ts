export interface FileEntry {
  path: string;
  filename: string;
  extension: string;
  date_source: "exif" | "filesystem" | "none";
  datetime: string | null;
  is_heic: boolean;
}

export interface PreviewEntry {
  original_path: string;
  original_name: string;
  new_name: string;
  warning: string | null;
}

export interface RenameResult {
  success_count: number;
  error_count: number;
  errors: { filename: string; reason: string }[];
}

export interface ProgressPayload {
  current: number;
  total: number;
  filename: string;
}

export interface ToolsStatus {
  exiftool: boolean;
  heif_convert: boolean;
}

export type RenameFormat =
  | "YYYY_MM_DD__hhmmss"
  | "YYMMDD_hhmmss"
  | "YYMMDD_original"
  | "NO_RENAME";

export interface AppState {
  files: FileEntry[];
  previews: PreviewEntry[];
  format: RenameFormat;
  offsetSeconds: number;
  createBackup: boolean;
  convertHeic: boolean;
  selectedIndices: Set<number>;
  isProcessing: boolean;
  canUndo: boolean;
  tools: ToolsStatus;
}
