import { gql, useMutation } from '@apollo/client';
import { Button, Classes, Dialog, FormGroup, HTMLSelect, InputGroup, Intent, Switch } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';

import { Action } from '../types';

const ASSIGN_PERMISSION = gql`
  mutation AssignPermission($user: String!, $path: String!, $action: Action!, $affectsChildren: Boolean!) {
    assignPermissionToUser(user: $user, path: $path, action: $action, affectsChildren: $affectsChildren) {
      id
    }
  }
`;

interface AssignPermissionVariables {
  user: string;
  path: string;
  action: Action;
  affectsChildren: boolean;
}

const ACTION_SELECT_OPTIONS = [
  { label: 'Admin access', value: Action.Admin },
  { label: 'Allow modification', value: Action.Modify },
  { label: 'Read-only', value: Action.Read },
  { label: 'No access', value: Action.Deny },
];

interface Props {
  user: string;
}

const AssignPermission = ({ user }: Props): JSX.Element => {
  const [assign, { loading }] = useMutation<void, AssignPermissionVariables>(ASSIGN_PERMISSION, {
    refetchQueries: ['GetUser'],
  });

  const [isOpen, setOpen] = useState(false);

  const [path, setPath] = useState('');
  const [action, setAction] = useState(Action.Modify);
  const [affectsChildren, setAffectsChildren] = useState(false);

  useEffect(() => {
    if (!loading) setOpen(false);
  }, [loading]);

  return (
    <>
      <Dialog
        isOpen={isOpen}
        icon="plus"
        title="Assign a new permission"
        canOutsideClickClose={true}
        canEscapeKeyClose={true}
      >
        <div className={Classes.DIALOG_BODY}>
          <FormGroup
            label="Path"
            labelFor="path-input"
            labelInfo="(required)"
            helperText="The path to apply the permission to"
          >
            <InputGroup
              id="path-input"
              placeholder="/some/path"
              value={path}
              onChange={(e) => setPath(e.target.value)}
            />
          </FormGroup>
          <FormGroup label="Action" labelFor="action-select">
            <HTMLSelect
              id="action-select"
              options={ACTION_SELECT_OPTIONS}
              value={action}
              onChange={(e) => setAction(e.target.value as Action)}
            />
          </FormGroup>
          <Switch
            label="Should this apply to sub-paths?"
            innerLabel="no"
            innerLabelChecked="yes"
            checked={affectsChildren}
            onChange={() => setAffectsChildren(!affectsChildren)}
          />
        </div>
        <div className={Classes.DIALOG_FOOTER}>
          <div className={Classes.DIALOG_FOOTER_ACTIONS}>
            <Button outlined text="Nevermind" disabled={loading} onClick={() => setOpen(false)} />
            <Button
              intent={Intent.SUCCESS}
              loading={loading}
              text="Assign"
              onClick={() => assign({ variables: { user, path, action, affectsChildren } })}
            />
          </div>
        </div>
      </Dialog>

      <Button intent={Intent.SUCCESS} outlined icon="plus" text="Assign permission" onClick={() => setOpen(true)} />
    </>
  );
};

export default AssignPermission;
