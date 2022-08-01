import { gql, useMutation, useQuery } from '@apollo/client';
import { Alert, Button, Callout, H1, H5, Intent, Pre, Text } from '@blueprintjs/core';
import React, { useEffect, useState } from 'react';

import { Details, Row } from '@components/Details';
import { usePageTitle } from '@lib/hooks';
import { success } from '@lib/toasts';
import { User } from '@lib/types';

import styles from './style.module.css';

const DAV_URL = `${import.meta.env.VITE_BASE_URL || window.origin}/dav`.replace('http', 'dav');

const GET_DETAILED_PROFILE = gql`
  query GetDetailedProfile {
    me {
      name
      username
    }
  }
`;

interface DetailedProfile {
  me: Pick<User, 'name' | 'username'>;
}

const REGENERATE_ACCESS_TOKEN = gql`
  mutation RegenerateAccessToken {
    regenerateAccessToken {
      token
    }
  }
`;

interface RegenerateAccessToken {
  regenerateAccessToken: {
    token: string;
  };
}

const Profile = (): JSX.Element => {
  const { data } = useQuery<DetailedProfile>(GET_DETAILED_PROFILE);
  const [regenerate, { data: regenerateData, loading: regenerateLoading }] =
    useMutation<RegenerateAccessToken>(REGENERATE_ACCESS_TOKEN);

  const [isOpen, setOpen] = useState(false);

  usePageTitle('Profile');

  // Close the re-generate dialog once complete
  useEffect(() => {
    if (!regenerateLoading) setOpen(false);
  }, [regenerateLoading]);

  const onCopy = async () => {
    await navigator.clipboard.writeText(regenerateData?.regenerateAccessToken.token || '');
    success('Copied access token to clipboard');
  };

  return (
    <>
      <H1>Profile</H1>
      <Details>
        <Row label="Name" value={data?.me.name} />
        <Row label="Username" value={data?.me.username} />
        <Row label="Access Token">
          <Button
            className={styles.regenerateButton}
            small
            intent={Intent.WARNING}
            icon="refresh"
            text="Re-generate"
            loading={regenerateLoading}
            onClick={() => setOpen(true)}
          />
        </Row>
      </Details>

      {regenerateData && (
        <Callout className={styles.callout} intent={Intent.WARNING} title="Your access token">
          <div className={styles.generatedToken}>
            <Pre className={styles.token}>{regenerateData.regenerateAccessToken.token}</Pre>
            <Button className={styles.copy} large outlined icon="clipboard" onClick={onCopy} />
          </div>
          Save this somewhere safe, it will only be shown once!
        </Callout>
      )}

      <Callout className={styles.callout} intent={Intent.PRIMARY} title="Connecting with WebDAV">
        To connect using WebDAV, you will need the server&apos;s URL, your username, and your access token. Note that
        your password is not the same as your account&apos;s password.
        <br />
        <br />
        <H5>Connection Details:</H5>
        <Pre>
          URL: {DAV_URL}
          <br />
          Username: {data?.me.username || '...'}
          <br />
          Password: *****
        </Pre>
        To view your password, you must re-generate your access token.
      </Callout>

      <Alert
        cancelButtonText="Nevermind"
        confirmButtonText="Re-generate"
        icon="warning-sign"
        intent={Intent.WARNING}
        canOutsideClickCancel={true}
        canEscapeKeyCancel={true}
        loading={regenerateLoading}
        isOpen={isOpen}
        onConfirm={() => regenerate()}
        onCancel={() => setOpen(false)}
      >
        <Text>
          Are you sure you want to re-generate your access token? Any existing sessions using your previous token will
          be made invalid.
        </Text>
      </Alert>
    </>
  );
};

export default Profile;
