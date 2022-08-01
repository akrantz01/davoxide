import { gql, useQuery } from '@apollo/client';
import { Classes, H1, H3, Icon, Text } from '@blueprintjs/core';
import React, { useEffect } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';

import { Details, Row } from '@components/Details';
import { NonIdealRow, Table } from '@components/Table';
import { usePageTitle } from '@lib/hooks';
import { danger, warning } from '@lib/toasts';
import { User } from '@lib/types';

import AssignPermission from './components/AssignPermission';
import DeleteButton from './components/DeleteButton';
import EditableDefaultAccess from './components/EditableDefaultAccess';
import PermissionRow from './components/PermissionRow';
import styles from './style.module.css';

const GET_USER = gql`
  query GetUser($username: String!) {
    user(username: $username) {
      name
      username
      defaultAccess
      permissions {
        id
        action
        path
        affectsChildren
      }
    }
  }
`;

interface GetUser {
  user: User;
}

interface GetUserVariables {
  username: string;
}

const UserDetail = (): JSX.Element => {
  const { username = '' } = useParams();
  usePageTitle(`Admin - ${username}`);

  const navigate = useNavigate();
  const { loading, data, error } = useQuery<GetUser, GetUserVariables>(GET_USER, { variables: { username } });

  // Handle errors
  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
      case 'not found':
        warning('The requested user could not be found');
        navigate('/admin');
        break;

      case 'permission denied':
        danger('You do not have permissions to access this page');
        navigate('/');
        break;

      default:
        danger('An unknown error occurred');
        console.error(error.message);
        break;
    }
  }, [loading, error]);

  const permissions = data?.user.permissions || [];

  return (
    <>
      <H1>User - {username}</H1>

      <Details>
        <Row label="Name" value={data?.user.name} />
        <Row label="Username" value={data?.user.username} />
        <Row label="Default Access">
          <EditableDefaultAccess
            className={styles.defaultAccessButton}
            user={username}
            action={data?.user.defaultAccess}
          />
        </Row>
        {/* TODO: show access token status */}
      </Details>

      <div className={styles.permissionsWrapper}>
        <div className={styles.permissionsHeader}>
          <H3>Permissions:</H3>
          <AssignPermission user={username} />
        </div>

        <Table
          className={styles.permissionsTable}
          headers={['Path', 'Action', 'Includes Children?']}
          hasActions
          loading={loading}
        >
          {permissions.length === 0 && (
            <NonIdealRow
              icon="help"
              title="No permissions found"
              description="Get started by assigning a new permission to the user."
            />
          )}
          {permissions.map((p) => (
            <PermissionRow key={p.id} {...p} />
          ))}
        </Table>
      </div>

      <div className={styles.actions}>
        <Link to="/admin" className={Classes.BUTTON}>
          <Icon icon="arrow-left" />
          <Text>Back</Text>
        </Link>
        <DeleteButton user={username} />
      </div>
    </>
  );
};

export default UserDetail;
