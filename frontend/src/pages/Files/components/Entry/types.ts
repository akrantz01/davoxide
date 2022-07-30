export enum Type {
  Directory = 'DIRECTORY',
  File = 'FILE',
  Unknown = 'UNKNOWN',
}

export interface DirectoryEntry {
  type: Type;
  name: string;
  lastModified: string;
  size: number;
}
