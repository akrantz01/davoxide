import { gql, useMutation } from '@apollo/client';
import { Alert, Button, Intent, Text } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

import { danger, success } from '@lib/toasts';

const DELETE_USER = gql`
  mutation DeleteUser($user: String!) {
    deleteUser(user: $user) {
      lastRemoved
    }
  }
`;

interface DeleteUserVariables {
  user: string;
}

interface Props {
  user: string;
}

const DeleteButton = ({ user }: Props): JSX.Element => {
  const navigate = useNavigate();

  const [remove, { loading, data, error }] = useMutation<void, DeleteUserVariables>(DELETE_USER, {
    refetchQueries: ['ListUsers'],
  });
  const [isOpen, setOpen] = useState(false);

  useEffect(() => {
    if (!loading) setOpen(false);
    if (loading || !data) return;

    success(`Successfully deleted user ${user}`);
    navigate('/admin');
  }, [loading]);

  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
      case 'cannot delete yourself':
        danger('You cannot delete yourself');
        break;

      default:
        danger('An unexpected error occurred');
        console.log(error.message);
        break;
    }
  }, [loading, error]);

  return (
    <>
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
        onConfirm={() => remove({ variables: { user } })}
      >
        <Text>
          Are you sure you want to delete this user? Deleting the user will not revoke all their access. If they still
          have access through the SSO, they will still be able to login.
        </Text>
      </Alert>

      <Button intent={Intent.DANGER} outlined icon="trash" text="Delete" onClick={() => setOpen(true)} />
    </>
  );
};

export default DeleteButton;
