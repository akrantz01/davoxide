import { Button, H1, Text } from '@blueprintjs/core';
import React from 'react';
import { useNavigate } from 'react-router-dom';

import './style.css';

const NotFound = (): JSX.Element => {
  const navigate = useNavigate();

  return (
    <div>
      <H1>Not Found</H1>
      <Text className="bp4-text-large">
        The requested URL could not be found. Please check it is correct and try again.
      </Text>
      <Button type="button" className="back-button" onClick={() => navigate(-1)}>
        Back
      </Button>
    </div>
  );
};

export default NotFound;
