export enum Action {
  Admin = 'ADMIN',
  Modify = 'MODIFY',
  Read = 'READ',
  Deny = 'DENY',
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
  permissions: Permission[];
}
