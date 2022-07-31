import { gql, useQuery } from '@apollo/client';
import { Classes, H1, H3, H5, HTMLTable, Icon, NonIdealState, Spinner, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import React, { ReactNode, useEffect } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';

import Toaster from '../../toasts';
import AssignPermission from './components/AssignPermission';
import DeleteButton from './components/DeleteButton';
import EditableDefaultAccess from './components/EditableDefaultAccess';
import PermissionRow from './components/PermissionRow';
import { User } from './types';

import './style.css';

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

interface DetailProps {
  label: string;
  value?: ReactNode;
}

const Detail = ({ label, value }: DetailProps): JSX.Element => (
  <div className="detail">
    <H5>{label}:</H5>
    <Text className={classNames(Classes.TEXT_LARGE, 'detail-value')}>{value ? value : <Spinner size={20} />}</Text>
  </div>
);

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

const UserDetail = (): JSX.Element => {
  const navigate = useNavigate();
  const { username = '' } = useParams();
  const { loading, data, error } = useQuery<GetUser, GetUserVariables>(GET_USER, { variables: { username } });

  // Set the page title
  useEffect(() => {
    document.title = `DAVOxide - Admin - ${username}`;
  }, []);

  // Handle errors
  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
      case 'not found':
        Toaster.show({ message: 'The requested user could not be found', intent: 'warning', timeout: 2500 });
        navigate('/admin');
        break;

      case 'permission denied':
        Toaster.show({
          message: 'You do not have permissions to access this page',
          intent: 'danger',
          timeout: 2500,
        });
        navigate('/');
        break;

      default:
        Toaster.show({ message: 'An unknown error occurred', intent: 'danger' });
        console.error(error.message);
        break;
    }
  }, [loading, error]);

  const permissions = data?.user.permissions || [];

  return (
    <>
      <H1>User - {username}</H1>

      <div className="user-details">
        <Detail label="Name" value={data?.user.name} />
        <Detail label="Username" value={data?.user.username} />
        <Detail
          label="Default Access"
          value={<EditableDefaultAccess user={username} action={data?.user.defaultAccess} />}
        />
        {/* TODO: show access token status */}
      </div>

      <div className="user-permissions">
        <div className="permissions-header">
          <H3>Permissions:</H3>
          <AssignPermission user={username} />
        </div>

        <HTMLTable className="permissions-table">
          <thead>
            <tr>
              <th scope="col">Path</th>
              <th scope="col">Action</th>
              <th scope="col">Includes Children?</th>
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
            {!loading && permissions.length === 0 && (
              <CenteredRow>
                <NonIdealState
                  icon="help"
                  title="No permissions found"
                  description="Get started by assigning a new permission to the user."
                />
              </CenteredRow>
            )}
            {permissions.map((p) => (
              <PermissionRow key={p.id} {...p} />
            ))}
          </tbody>
        </HTMLTable>
      </div>

      <div className="bottom-actions">
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
