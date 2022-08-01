import { gql, useMutation } from '@apollo/client';
import { Button, Classes, Dialog, FormGroup, HTMLSelect, Intent } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';

import ActionTag from '@components/ActionTag';
import { danger } from '@lib/toasts';
import { ACTION_SELECT_OPTIONS, Action } from '@lib/types';

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

interface Props {
  user: string;
  action?: Action;
  className?: string;
}

const EditableDefaultAccess = ({ action: defaultAction, user, className }: Props): JSX.Element => {
  const [update, { loading, error }] = useMutation<void, UpdateDefaultPermissionVariables>(UPDATE_DEFAULT_PERMISSION, {
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

  useEffect(() => {
    if (loading || !error) return;

    danger('An unexpected error occurred');
    console.error(error.message);
  }, [loading, error]);

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

      <ActionTag
        className={className}
        action={defaultAction}
        interactive
        rightIcon="edit"
        onClick={() => setOpen(true)}
      />
    </>
  );
};

export default EditableDefaultAccess;
