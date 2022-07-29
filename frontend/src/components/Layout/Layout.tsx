import { Card, Elevation } from '@blueprintjs/core';
import React, { ReactNode } from 'react';

import Navigation from './components/Navigation';

import './style.css';

interface Props {
  children?: ReactNode;
}

export const Layout = ({ children }: Props): JSX.Element => {
  return (
    <>
      <Navigation />

      <Card elevation={Elevation.TWO} className="card">
        {children}
      </Card>
    </>
  );
};
