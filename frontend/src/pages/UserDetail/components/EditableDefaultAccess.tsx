import { gql, useMutation } from '@apollo/client';
import { Button, Classes, Dialog, FormGroup, HTMLSelect, Intent } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';

import { Action } from '../types';
import ActionTag from './ActionTag';

const UPDATE_DEFAULT_PERMISSION = gql`
  mutation UpdateDefaultAccess($user: String!, $action: Action!) {
    updateDefaultPermission(user: $user, action: $action) {
      username
      defaultAccess
    }
  }
`;

interface UpdateDefaultPermissionVariables {
  user: string;
  action: Action;
}

const ACTION_SELECT_OPTIONS = [
  { label: 'Admin access', value: Action.Admin },
  { label: 'Allow modification', value: Action.Modify },
  { label: 'Read-only', value: Action.Read },
  { label: 'No access', value: Action.Deny },
];

interface Props {
  user: string;
  action?: Action;
}

const EditableDefaultAccess = ({ action: defaultAction, user }: Props): JSX.Element => {
  const [update, { loading }] = useMutation<void, UpdateDefaultPermissionVariables>(UPDATE_DEFAULT_PERMISSION, {
    refetchQueries: ['GetUser'],
  });
  const [isOpen, setOpen] = useState(false);

  const [action, setAction] = useState(defaultAction || Action.Modify);
  useEffect(() => {
    setAction(defaultAction || Action.Modify);
  }, [defaultAction]);

  useEffect(() => {
    if (loading) setOpen(false);
  }, [loading]);

  return (
    <>
      <Dialog
        isOpen={isOpen}
        icon="edit"
        title="Change default access permission"
        canOutsideClickClose={true}
        canEscapeKeyClose={true}
      >
        <div className={Classes.DIALOG_BODY}>
          <FormGroup label="Action" labelFor="action-select">
            <HTMLSelect
              id="action-select"
              options={ACTION_SELECT_OPTIONS}
              value={action}
              onChange={(e) => setAction(e.target.value as Action)}
            />
          </FormGroup>
        </div>
        <div className={Classes.DIALOG_FOOTER}>
          <div className={Classes.DIALOG_FOOTER_ACTIONS}>
            <Button outlined text="Nevermind" disabled={loading} onClick={() => setOpen(false)} />
            <Button
              intent={Intent.SUCCESS}
              loading={loading}
              text="Update"
              onClick={() => update({ variables: { user, action } })}
            />
          </div>
        </div>
      </Dialog>

      <ActionTag action={defaultAction} interactive rightIcon="edit" onClick={() => setOpen(true)} />
    </>
  );
};

export default EditableDefaultAccess;
