import { Intent, Tag, TagProps } from '@blueprintjs/core';
import React from 'react';

import { Action } from '@lib/types';

const decodeIntent = (action?: Action): Intent | undefined => {
  if (!action) return undefined;

  switch (action) {
    case Action.Admin:
      return Intent.DANGER;
    case Action.Modify:
    case Action.Read:
      return Intent.PRIMARY;
    case Action.Deny:
      return Intent.WARNING;
    default:
      throw new TypeError(`unknown permission: ${action}`);
  }
};

interface Props extends Omit<TagProps, 'intent'> {
  action?: Action;
}

const ActionTag = ({ action, ...props }: Props): JSX.Element => (
  <Tag intent={decodeIntent(action)} {...props}>
    {action}
  </Tag>
);

export default ActionTag;
