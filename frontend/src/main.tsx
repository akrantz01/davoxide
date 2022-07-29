import { ApolloProvider } from '@apollo/client';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { BrowserRouter, Route, Routes } from 'react-router-dom';

import 'normalize.css/normalize.css';
import '@blueprintjs/core/lib/css/blueprint.css';
import '@blueprintjs/icons/lib/css/blueprint-icons.css';

import { client } from './api';
import App from './App';
import Layout from './components/Layout';
import NotFound from './pages/NotFound';

const root = document.getElementById('root') as HTMLElement;
ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <BrowserRouter>
        <Layout>
          <Routes>
            <Route index element={<App />} />
            <Route path="*" element={<NotFound />} />
          </Routes>
        </Layout>
      </BrowserRouter>
    </ApolloProvider>
  </React.StrictMode>,
);
