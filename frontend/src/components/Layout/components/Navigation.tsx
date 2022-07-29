import { Alignment, Icon, Navbar, Text } from '@blueprintjs/core';
import React from 'react';
import { NavLink } from 'react-router-dom';

import './navigation.css';

const Navigation = (): JSX.Element => {
  return (
    <Navbar>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>DAVoxide</Navbar.Heading>
        <Navbar.Divider />
        <NavLink to="/" role="button" className="bp4-button bp4-minimal">
          <Icon icon="home" />
          <Text>Home</Text>
        </NavLink>
        <NavLink to="/admin" role="button" className="bp4-button bp4-minimal">
          <Icon icon="settings" />
          <Text>Admin</Text>
        </NavLink>
      </Navbar.Group>
      <Navbar.Group align={Alignment.RIGHT} className="visible-sm">
        <Icon icon="person" className="icon" />
        <Text>Alex Krantz</Text>
      </Navbar.Group>
    </Navbar>
  );
};

export default Navigation;
