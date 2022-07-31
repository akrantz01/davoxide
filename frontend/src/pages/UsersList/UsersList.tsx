import { gql, useQuery } from '@apollo/client';
import { H1, HTMLTable, NonIdealState, Spinner } from '@blueprintjs/core';
import React, { ReactNode, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';

import ActionTag from '@components/ActionTag';
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

interface CenteredRowProps {
  children: ReactNode;
}

const CenteredRow = ({ children }: CenteredRowProps): JSX.Element => (
  <tr>
    <td colSpan={4}>
      <div className="centered-row">{children}</div>
    </td>
  </tr>
);

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
      <HTMLTable className="users">
        <thead>
          <tr>
            <th scope="col">Name</th>
            <th scope="col">Username</th>
            <th scope="col">Default Permission</th>
            <th scope="col" className="hidden-column">
              <span>Actions</span>
            </th>
          </tr>
        </thead>
        <tbody>
          {loading && (
            <CenteredRow>
              <Spinner />
            </CenteredRow>
          )}
          {users.length === 0 && (
            <CenteredRow>
              <NonIdealState
                icon="user"
                title="No users found"
                description="We couldn't find any registered users. Something went horribly wrong."
              />
            </CenteredRow>
          )}
          {users.map((user) => (
            <tr key={user.username}>
              <td>{user.name}</td>
              <td>{user.username}</td>
              <td>
                <ActionTag action={user.defaultAccess} />
              </td>
              <td className="actions">
                <Link to={user.username}>Details</Link>
              </td>
            </tr>
          ))}
        </tbody>
      </HTMLTable>
    </>
  );
};

export default UsersList;
