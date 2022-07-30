import { Classes, Icon, IconName, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import fileSize from 'filesize';
import { DateTime } from 'luxon';
import React from 'react';
import { Link } from 'react-router-dom';

import { DirectoryEntry, Type } from './types';

import './style.css';

const BASE_URL = import.meta.env.VITE_BASE_URL || window.origin;

const iconForType = (type: Type): IconName => {
  switch (type) {
    case Type.Directory:
      return 'folder-close';
    case Type.File:
      return 'document';
    case Type.Unknown:
      return 'help';
    default:
      throw new TypeError(`invalid type for directory entry: ${type}`);
  }
};

const Entry = ({ type, name, path, lastModified, size }: DirectoryEntry): JSX.Element => {
  const icon = iconForType(type);

  return (
    <div className="entry">
      <div className="label">
        <Icon icon={icon} />
        {type === Type.Directory ? (
          <Link to={'/files/' + path} className="label-content">
            {name}
          </Link>
        ) : (
          <Text className="label-content">{name}</Text>
        )}
      </div>
      <Text className="last-modified">{DateTime.fromISO(lastModified).toLocaleString(DateTime.DATETIME_MED)}</Text>
      {type === Type.File && (
        <div className="actions">
          <span>{fileSize(size, { base: 2 })}</span>
          <a href={BASE_URL + '/dav/' + path} className={classNames(Classes.BUTTON, Classes.SMALL, Classes.MINIMAL)}>
            <Icon icon="cloud-download" />
          </a>
        </div>
      )}
    </div>
  );
};

export default Entry;
