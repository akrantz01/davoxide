import { NonIdealState, NonIdealStateProps } from '@blueprintjs/core';
import React from 'react';

import CenteredRow from './CenteredRow';

interface Props extends NonIdealStateProps {
  span?: number;
}

const NonIdealRow = ({ span, ...props }: Props): JSX.Element => (
  <CenteredRow span={span}>
    <NonIdealState {...props} />
  </CenteredRow>
);

export default NonIdealRow;
