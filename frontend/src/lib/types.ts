export enum Action {
  Admin = 'ADMIN',
  Modify = 'MODIFY',
  Read = 'READ',
  Deny = 'DENY',
}

export const ACTION_SELECT_OPTIONS = [
  { label: 'Admin access', value: Action.Admin },
  { label: 'Allow modification', value: Action.Modify },
  { label: 'Read-only', value: Action.Read },
  { label: 'No access', value: Action.Deny },
];

export enum EntryType {
  Directory = 'DIRECTORY',
  File = 'FILE',
  Unknown = 'UNKNOWN',
}

export interface Entry {
  type: EntryType;
  name: string;
  path: string;
  createdAt: string;
  lastModified: string;
  size: number;
}

export interface Permission {
  id: number;
  action: Action;
  path: string;
  affectsChildren: boolean;
}

export interface User {
  name: string;
  username: string;
  defaultAccess: Action;
  hasAccessToken: boolean;
  permissions: Permission[];
}
