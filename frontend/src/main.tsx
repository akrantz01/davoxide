import { ApolloProvider } from '@apollo/client';
import React, { lazy } from 'react';
import { createRoot } from 'react-dom/client';
import { BrowserRouter, Route, Routes } from 'react-router-dom';

import 'normalize.css/normalize.css';
import '@blueprintjs/core/lib/css/blueprint.css';
import '@blueprintjs/icons/lib/css/blueprint-icons.css';
import '@blueprintjs/popover2/lib/css/blueprint-popover2.css';

import { client } from '@lib/api';

import Layout from './components/Layout';

const Files = lazy(() => import('./pages/Files'));
const NotFound = lazy(() => import('./pages/NotFound'));
const Profile = lazy(() => import('./pages/Profile'));
const UserDetail = lazy(() => import('./pages/UserDetail'));
const UsersList = lazy(() => import('./pages/UsersList'));

const root = document.getElementById('root') as HTMLElement;
createRoot(root).render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <BrowserRouter>
        <Layout>
          <Routes>
            <Route index element={<Files />} />
            <Route path="/files/*" element={<Files />} />
            <Route path="/profile" element={<Profile />} />
            <Route path="/admin" element={<UsersList />} />
            <Route path="/admin/:username" element={<UserDetail />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </Layout>
      </BrowserRouter>
    </ApolloProvider>
  </React.StrictMode>,
);
