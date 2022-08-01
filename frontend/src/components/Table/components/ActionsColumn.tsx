import React, { ReactNode } from 'react';

interface Props {
  children?: ReactNode;
}

const ActionsColumn = ({ children }: Props): JSX.Element => (
  <td style={{ position: 'relative', textAlign: 'right' }}>{children}</td>
);

export default ActionsColumn;
