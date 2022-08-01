import React, { ReactNode } from 'react';

interface Props {
  children: ReactNode;
  span?: number;
}

const CenteredRow = ({ children, span = 4 }: Props): JSX.Element => (
  <tr>
    <td colSpan={span}>
      <div style={{ marginTop: '2rem' }}>{children}</div>
    </td>
  </tr>
);

export default CenteredRow;
