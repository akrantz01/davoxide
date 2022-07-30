import { Button, Classes, Icon, Text } from '@blueprintjs/core';
import React from 'react';
import { Link } from 'react-router-dom';

interface Props {
  to: string;
  disabled?: boolean;
}

const BackButton = ({ to, disabled }: Props): JSX.Element => {
  if (disabled) return <Button disabled={disabled} icon="arrow-left" text="Back" />;

  return (
    <Link to={to} className={Classes.BUTTON}>
      <Icon icon="arrow-left" />
      <Text>Back</Text>
    </Link>
  );
};

export default BackButton;
