import { gql, useMutation } from '@apollo/client';
import { Alert, Button, Intent, Position, Tag, Text } from '@blueprintjs/core';
import { Tooltip2 } from '@blueprintjs/popover2';
import React, { useEffect, useState } from 'react';

import ActionTag from '@components/ActionTag';
import { ActionsColumn } from '@components/Table';
import { danger } from '@lib/toasts';
import { Permission } from '@lib/types';

const UNASSIGN_PERMISSION = gql`
  mutation UnassignPermission($id: Int!) {
    removePermission(permissionId: $id) {
      lastRemoved
    }
  }
`;

interface UnassignPermissionVariables {
  id: number;
}

const PermissionRow = (permission: Permission): JSX.Element => {
  const [unassign, { loading, error }] = useMutation<void, UnassignPermissionVariables>(UNASSIGN_PERMISSION, {
    refetchQueries: ['GetUser'],
  });
  const [isOpen, setOpen] = useState(false);

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
      <tr>
        <td>{permission.path}</td>
        <td>
          <ActionTag action={permission.action} />
        </td>
        <td>
          <Tag intent={permission.affectsChildren ? Intent.SUCCESS : Intent.DANGER}>
            {permission.affectsChildren ? 'Yes' : 'No'}
          </Tag>
        </td>
        <ActionsColumn>
          <Tooltip2 content="Delete" intent={Intent.DANGER} position={Position.LEFT}>
            <Button icon="trash" intent={Intent.DANGER} minimal onClick={() => setOpen(true)} />
          </Tooltip2>
        </ActionsColumn>
      </tr>

      <Alert
        isOpen={isOpen}
        cancelButtonText="Nevermind"
        confirmButtonText="Delete"
        icon="trash"
        intent={Intent.DANGER}
        canOutsideClickCancel={true}
        canEscapeKeyCancel={true}
        loading={loading}
        onCancel={() => setOpen(false)}
        onConfirm={() => unassign({ variables: { id: permission.id } })}
      >
        <Text>Are you sure you want to delete this permission? This action is permanent and cannot be undone.</Text>
      </Alert>
    </>
  );
};

export default PermissionRow;
