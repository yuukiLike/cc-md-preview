export interface CacheInfo {
  profile: string;
  categories: CacheCategory[];
  totalSize: number;
  totalFiles: number;
}

export interface CacheCategory {
  name: string;
  path: string;
  size: number;
  fileCount: number;
  exists: boolean;
}

export interface CacheEntry {
  name: string;
  size: number;
  modified: string;
}

export interface CleanResult {
  deletedFiles: number;
  freedBytes: number;
  errors: string[];
}

export interface BrowserInfo {
  name: string;
  installed: boolean;
}
