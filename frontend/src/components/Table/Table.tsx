import { HTMLTable, Spinner } from '@blueprintjs/core';
import React, { ReactNode } from 'react';

import CenteredRow from '@components/Table/components/CenteredRow';

import styles from './style.module.css';

interface Props {
  headers: string[];
  hasActions?: boolean;
  className?: string;
  loading?: boolean;
  children?: ReactNode;
}

const Table = ({ className, headers, hasActions, loading, children }: Props): JSX.Element => {
  const spinner = (
    <CenteredRow>
      <Spinner />
    </CenteredRow>
  );

  return (
    <HTMLTable className={className}>
      <thead>
        {headers.map((h, i) => (
          <th key={i} scope="col">
            {h}
          </th>
        ))}
        {hasActions && (
          <th scope="col" className={styles.actions}>
            <span className={styles.label}>Actions</span>
          </th>
        )}
      </thead>
      <tbody>{loading ? spinner : children}</tbody>
    </HTMLTable>
  );
};

export default Table;
