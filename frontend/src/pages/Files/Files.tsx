import { gql, useQuery } from '@apollo/client';
import { BreadcrumbProps, H1, NonIdealState, Spinner } from '@blueprintjs/core';
import { Breadcrumbs2 } from '@blueprintjs/popover2';
import React, { useEffect } from 'react';
import { useNavigate, useParams } from 'react-router-dom';

import Toaster from '../../toasts';
import BackButton from './components/BackButton';
import Breadcrumb from './components/Breadcrumb';
import Entry, { DirectoryEntry, Type } from './components/Entry';

import './style.css';

const LIST_DIRECTORY = gql`
  query ListDirectory($path: String) {
    listDirectory(path: $path) {
      type
      name
      path
      lastModified
      size
    }
  }
`;

interface ListDirectory {
  listDirectory: DirectoryEntry[];
}

interface ListDirectoryVariables {
  path?: string;
}

const usePath = (): string => {
  const { '*': path } = useParams();
  return path || '/';
};

const generateBreadcrumbs = (path: string): BreadcrumbProps[] => {
  if (path === '/') return [{ text: 'All Files', icon: 'folder-open' }];

  const crumbs: BreadcrumbProps[] = [{ text: 'All Files', icon: 'folder-open', href: '/files' }];

  const segments = path.split('/');
  for (let i = 0; i < segments.length; i++) {
    crumbs.push({
      text: segments[i],
      href: i === segments.length - 1 ? undefined : `/files/${segments.slice(0, i + 1).join('/')}`,
      current: i === segments.length - 1,
    });
  }

  return crumbs;
};

const orderEntries = (a: DirectoryEntry, b: DirectoryEntry): number => {
  if (a.name > b.name) return 1;
  else if (a.name < b.name) return -1;
  else return 0;
};

const Files = (): JSX.Element => {
  const navigate = useNavigate();
  const path = usePath();
  const { loading, error, data } = useQuery<ListDirectory, ListDirectoryVariables>(LIST_DIRECTORY, {
    variables: { path },
  });

  const previousDirectory = `/files/${path.split('/').slice(0, -1).join('/')}`;

  // Change the page title
  useEffect(() => {
    document.title = `DAVOxide - ${path === '/' ? path : '/' + path}`;
  }, [path]);

  // Handle errors
  useEffect(() => {
    if (loading || !error) return;

    switch (error.message) {
      // Navigate to the parent directory if not a directory
      case 'path is not a directory':
        Toaster.show({ message: 'The requested file is not a directory', intent: 'warning', timeout: 2500 });
        navigate(previousDirectory);
        break;

      case 'permission denied':
        Toaster.show({
          message: 'You do not have permissions to access this resource',
          intent: 'danger',
          timeout: 2500,
        });
        navigate('/files');
        break;

      case 'not found':
        Toaster.show({ message: 'The requested resource could not be found', intent: 'warning', timeout: 2500 });
        navigate('/files');
        break;

      default:
        Toaster.show({ message: 'An unknown error occurred', intent: 'danger' });
        console.error(error.message);
        break;
    }
  }, [loading, error]);

  // Order directories by name and files by name
  const entries = data?.listDirectory || [];
  const directories = entries.filter((e) => e.type === Type.Directory).sort(orderEntries);
  const files = entries.filter((e) => e.type !== Type.Directory).sort(orderEntries);

  return (
    <>
      <H1>Files</H1>
      <Breadcrumbs2 items={generateBreadcrumbs(path)} breadcrumbRenderer={Breadcrumb} />
      <div className="files">
        {(loading || !data) && <Spinner />}
        {directories.length !== 0 && directories.map((entry) => <Entry key={entry.name} {...entry} />)}
        {files.length !== 0 && files.map((entry) => <Entry key={entry.name} {...entry} />)}
        {directories.length === 0 && files.length === 0 && (
          <NonIdealState
            icon="search"
            title="There's nothing here"
            description="It looks like there are no files or folders in this directory. You can create some using the WebDAV interface."
          />
        )}
      </div>
      <BackButton to={previousDirectory} disabled={path === '/'} />
    </>
  );
};

export default Files;
