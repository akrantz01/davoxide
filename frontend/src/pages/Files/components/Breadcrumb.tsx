import { BreadcrumbProps, Classes, Icon } from '@blueprintjs/core';
import classNames from 'classnames';
import React from 'react';
import { Link } from 'react-router-dom';

// This is a re-implementation of the BlueprintJS Breadcrumb to use a React Router Link
const Breadcrumb = (props: BreadcrumbProps): JSX.Element => {
  const classes = classNames(
    Classes.BREADCRUMB,
    {
      [Classes.BREADCRUMB_CURRENT]: props.current,
      [Classes.DISABLED]: props.disabled,
    },
    props.className,
  );

  const icon = props.icon != null ? <Icon icon={props.icon} title={props.iconTitle} /> : undefined;

  if (props.href) {
    return (
      <Link to={props.href} className={classes} tabIndex={props.disabled ? undefined : 0} target={props.target}>
        {icon}
        {props.text}
        {props.children}
      </Link>
    );
  } else {
    return (
      <span className={classes}>
        {icon}
        {props.text}
        {props.children}
      </span>
    );
  }
};

export default Breadcrumb;
