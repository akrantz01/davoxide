import { gql, useQuery } from '@apollo/client';
import { H1 } from '@blueprintjs/core';
import React, { useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';

import ActionTag from '@components/ActionTag';
import { ActionsColumn, NonIdealRow, Table } from '@components/Table';
import { usePageTitle } from '@lib/hooks';
import { danger } from '@lib/toasts';
import { User } from '@lib/types';

import './style.css';

const LIST_USERS = gql`
  query ListUsers {
    users {
      name
      username
      defaultAccess
    }
  }
`;

interface ListUsers {
  users: Omit<User, 'permissions'>[];
}

const UsersList = (): JSX.Element => {
  const navigate = useNavigate();
  const { data, error, loading } = useQuery<ListUsers>(LIST_USERS);

  usePageTitle('Admin');

  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
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

  const users = data?.users || [];

  return (
    <>
      <H1>Admin</H1>
      <Table headers={['Name', 'Username', 'Default Permission']} className="users" hasActions loading={loading}>
        {users.length === 0 && (
          <NonIdealRow
            icon="user"
            title="No users found"
            description="We couldn't find any registered users. Something went horribly wrong."
          />
        )}
        {users.map((user) => (
          <tr key={user.username}>
            <td>{user.name}</td>
            <td>{user.username}</td>
            <td>
              <ActionTag action={user.defaultAccess} />
            </td>
            <ActionsColumn>
              <Link to={user.username}>Details</Link>
            </ActionsColumn>
          </tr>
        ))}
      </Table>
    </>
  );
};

export default UsersList;
