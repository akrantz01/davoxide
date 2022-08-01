import { Classes, Icon, IconName, Position, Text } from '@blueprintjs/core';
import { Tooltip2 } from '@blueprintjs/popover2';
import classNames from 'classnames';
import fileSize from 'filesize';
import { DateTime } from 'luxon';
import React from 'react';
import { Link } from 'react-router-dom';

import { Entry as DirectoryEntry, EntryType } from '@lib/types';

import styles from './style.module.css';

const BASE_URL = import.meta.env.VITE_BASE_URL || window.origin;

const iconForType = (type: EntryType): IconName => {
  switch (type) {
    case EntryType.Directory:
      return 'folder-close';
    case EntryType.File:
      return 'document';
    case EntryType.Unknown:
      return 'help';
    default:
      throw new TypeError(`invalid type for directory entry: ${type}`);
  }
};

const Entry = ({ type, name, path, lastModified, size }: DirectoryEntry): JSX.Element => {
  const icon = iconForType(type);

  return (
    <div className={styles.wrapper}>
      <div className={styles.label}>
        <Icon icon={icon} />
        {type === EntryType.Directory ? (
          <Link to={'/files/' + path} className={styles.labelContent}>
            {name}
          </Link>
        ) : (
          <Text className={styles.labelContent}>{name}</Text>
        )}
      </div>
      <Text className={styles.lastModified}>
        {DateTime.fromISO(lastModified).toLocaleString(DateTime.DATETIME_MED)}
      </Text>
      {type === EntryType.File && (
        <div className={styles.actions}>
          <span>{fileSize(size, { base: 2 })}</span>
          <Tooltip2 content="Download" position={Position.LEFT}>
            <a href={BASE_URL + '/dav/' + path} className={classNames(Classes.BUTTON, Classes.SMALL, Classes.MINIMAL)}>
              <Icon icon="cloud-download" />
            </a>
          </Tooltip2>
        </div>
      )}
    </div>
  );
};

export default Entry;
