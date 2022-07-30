import { gql, useQuery } from '@apollo/client';
import { Alignment, Classes, Icon, Navbar, Spinner, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import React from 'react';
import { NavLink } from 'react-router-dom';

import './navigation.css';

const GET_PROFILE = gql`
  query GetProfile {
    me {
      name
      username
      defaultAccess
    }
  }
`;

interface Profile {
  me: {
    name: string;
    defaultAccess: string;
  };
}

const Navigation = (): JSX.Element => {
  const { loading, data } = useQuery<Profile>(GET_PROFILE);

  return (
    <Navbar>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>DAVoxide</Navbar.Heading>
        <Navbar.Divider />
        <NavLink to="/" role="button" className={classNames(Classes.BUTTON, Classes.MINIMAL)}>
          <Icon icon="home" />
          <Text>Home</Text>
        </NavLink>
        {data?.me.defaultAccess === 'ADMIN' && (
          <NavLink to="/admin" role="button" className={classNames(Classes.BUTTON, Classes.MINIMAL)}>
            <Icon icon="settings" />
            <Text>Admin</Text>
          </NavLink>
        )}
      </Navbar.Group>
      <Navbar.Group align={Alignment.RIGHT} className="visible-sm">
        <Icon icon="person" className="icon" />
        {loading || !data ? (
          <Spinner size={25} />
        ) : (
          <NavLink to="/profile" role="button" className={classNames(Classes.BUTTON, Classes.MINIMAL)}>
            <Text>{data.me.name}</Text>
          </NavLink>
        )}
      </Navbar.Group>
    </Navbar>
  );
};

export default Navigation;
