import { gql, useMutation, useQuery } from '@apollo/client';
import { Alert, Button, Callout, Classes, H1, H4, H5, Intent, Pre, Spinner, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import React, { useEffect, useState } from 'react';

import { usePageTitle } from '@lib/hooks';
import { User } from '@lib/types';

import './style.css';

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

interface DetailProps {
  label: string;
  value?: string;
}

const Detail = ({ label, value }: DetailProps): JSX.Element => (
  <div className="detail">
    <H4>{label}:</H4>
    <Text className={classNames(Classes.TEXT_LARGE, 'detail-value')}>{value ? value : <Spinner size={20} />}</Text>
  </div>
);

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

  return (
    <>
      <H1>Profile</H1>
      <div className="details">
        <Detail label="Name" value={data?.me.name} />
        <Detail label="Username" value={data?.me.username} />
        <div className="detail">
          <H4>Access Token:</H4>
          <Button
            className="regenerate-button"
            small
            intent={Intent.WARNING}
            icon="refresh"
            text="Re-generate"
            loading={regenerateLoading}
            onClick={() => setOpen(true)}
          />
        </div>
      </div>

      {regenerateData && (
        <Callout className="callout" intent={Intent.WARNING} title="Your access token">
          <Pre>{regenerateData.regenerateAccessToken.token}</Pre>
          Save this somewhere safe, it will only be shown once!
        </Callout>
      )}

      <Callout className="callout" intent={Intent.PRIMARY} title="Connecting with WebDAV">
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
