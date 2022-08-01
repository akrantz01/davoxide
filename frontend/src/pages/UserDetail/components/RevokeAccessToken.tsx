import { gql, useMutation } from '@apollo/client';
import { Alert, Button, Intent, Text } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';

import { danger } from '@lib/toasts';

const REVOKE_ACCESS_TOKEN = gql`
  mutation RevokeAccessToken($user: String!) {
    revokeAccessToken(username: $user) {
      username
    }
  }
`;

interface RevokeAccessTokenVariables {
  user: string;
}

interface Props {
  user: string;
}

const RevokeAccessToken = ({ user }: Props): JSX.Element => {
  const [revoke, { loading, error }] = useMutation<void, RevokeAccessTokenVariables>(REVOKE_ACCESS_TOKEN, {
    refetchQueries: ['GetUser'],
  });
  const [isOpen, setOpen] = useState(false);

  useEffect(() => {
    if (!loading) setOpen(false);
  }, [loading]);

  useEffect(() => {
    if (loading || !error) return;

    danger('An unexpected error occurred');
    console.error(error.message);
  }, [loading, error]);

  return (
    <>
      <Alert
        cancelButtonText="Nevermind"
        confirmButtonText="Revoke"
        icon="warning-sign"
        intent={Intent.WARNING}
        canOutsideClickCancel={true}
        canEscapeKeyCancel={true}
        loading={loading}
        isOpen={isOpen}
        onConfirm={() => revoke({ variables: { user } })}
        onCancel={() => setOpen(false)}
      >
        <Text>
          Are you sure you want to revoke {user}&apos;s access token? This will invalidate sessions using their previous
          token.
        </Text>
      </Alert>

      <Button
        style={{ marginLeft: '1rem', maxHeight: '1rem' }}
        small
        intent={Intent.WARNING}
        icon="remove"
        text="Revoke"
        loading={loading}
        onClick={() => setOpen(true)}
      />
    </>
  );
};

export default RevokeAccessToken;
