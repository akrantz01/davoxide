import { Classes, Icon, IconName, Text } from '@blueprintjs/core';
import classNames from 'classnames';
import fileSize from 'filesize';
import { DateTime } from 'luxon';
import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import urlJoin from 'url-join';

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

const buildFilePath = (file: string): string => {
  const { pathname } = useLocation();
  return urlJoin(pathname === '/' ? '/files' : pathname, file);
};

const Entry = ({ type, name, lastModified, size }: DirectoryEntry): JSX.Element => {
  const icon = iconForType(type);
  const path = buildFilePath(name);

  return (
    <div className="entry">
      <div className="label">
        <Icon icon={icon} />
        {type === Type.Directory ? (
          <Link to={path} className="label-content">
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
          <a
            href={BASE_URL + path.replace('/files', '/dav')}
            className={classNames(Classes.BUTTON, Classes.SMALL, Classes.MINIMAL)}
          >
            <Icon icon="cloud-download" />
          </a>
        </div>
      )}
    </div>
  );
};

export default Entry;
