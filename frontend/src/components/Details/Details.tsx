import { Classes, H4, Spinner, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import React, { ReactNode } from 'react';

import styles from './style.module.css';

interface DetailsProps {
  className?: string;
  children?: ReactNode;
}

const Details = ({ className, children }: DetailsProps): JSX.Element => (
  <div className={classNames(styles.wrapper, className)}>{children}</div>
);

interface RowProps {
  label: string;
  value?: string;
  children?: ReactNode;
}

const selectRendered = (value?: string, children?: ReactNode): ReactNode => {
  if (value) return <Text className={classNames(Classes.TEXT_LARGE, styles.value)}>{value}</Text>;
  else if (children) return children;
  else return <Spinner className={styles.spinner} size={20} />;
};

const Row = ({ label, value, children }: RowProps): JSX.Element => (
  <div className={styles.row}>
    <H4>{label}:</H4>
    {selectRendered(value, children)}
  </div>
);

export { Details, Row };
