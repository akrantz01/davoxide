import { Button, Classes, H1, Text } from '@blueprintjs/core';
import React from 'react';
import { useNavigate } from 'react-router-dom';

import styles from './style.module.css';

const NotFound = (): JSX.Element => {
  const navigate = useNavigate();

  return (
    <div>
      <H1>Not Found</H1>
      <Text className={Classes.TEXT_LARGE}>
        The requested URL could not be found. Please check it is correct and try again.
      </Text>
      <Button type="button" className={styles.backButton} onClick={() => navigate(-1)}>
        Back
      </Button>
    </div>
  );
};

export default NotFound;
