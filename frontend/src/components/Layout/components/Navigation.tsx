import { Alignment, Button, Icon, Navbar, Text } from '@blueprintjs/core';
import React from 'react';

import './navigation.css';

const Navigation = (): JSX.Element => {
  return (
    <Navbar>
      <Navbar.Group align={Alignment.LEFT}>
        <Navbar.Heading>DAVoxide</Navbar.Heading>
        <Navbar.Divider />
        <Button minimal icon="home">
          Home
        </Button>
        <Button minimal icon="settings">
          Admin
        </Button>
      </Navbar.Group>
      <Navbar.Group align={Alignment.RIGHT} className="visible-sm">
        <Icon icon="person" style={{ paddingRight: '0.5rem' }} />
        <Text>Alex Krantz</Text>
      </Navbar.Group>
    </Navbar>
  );
};

export default Navigation;
