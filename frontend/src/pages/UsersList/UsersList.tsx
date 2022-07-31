import { gql, useQuery } from '@apollo/client';
import { H1, HTMLTable, Intent, NonIdealState, Spinner, Tag } from '@blueprintjs/core';
import React, { ReactNode, useEffect } from 'react';
import { Link, useNavigate } from 'react-router-dom';

import './style.css';
import Toaster from '../../toasts';

const LIST_USERS = gql`
  query ListUsers {
    users {
      name
      username
      defaultAccess
    }
  }
`;

interface User {
  name: string;
  username: string;
  defaultAccess: string;
}

interface ListUsers {
  users: User[];
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

const intentForAccess = (access: string): Intent => {
  switch (access) {
    case 'ADMIN':
      return Intent.DANGER;
    case 'MODIFY':
    case 'READ':
      return Intent.PRIMARY;
    case 'DENY':
      return Intent.WARNING;
    default:
      throw new TypeError(`unknown permission: ${access}`);
  }
};

const UsersList = (): JSX.Element => {
  const navigate = useNavigate();
  const { data, error, loading } = useQuery<ListUsers>(LIST_USERS);

  // Set the page title
  useEffect(() => {
    document.title = 'DAVOxide - Admin';
  }, []);

  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
      case 'permission denied':
        Toaster.show({
          message: 'You do not have permissions to access this page',
          intent: Intent.DANGER,
          timeout: 2500,
        });
        navigate('/');
        break;

      default:
        Toaster.show({ message: 'An unknown error occurred', intent: Intent.DANGER, timeout: 2500 });
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
                <Tag intent={intentForAccess(user.defaultAccess)}>{user.defaultAccess}</Tag>
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
