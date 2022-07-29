import { ApolloClient, InMemoryCache } from '@apollo/client';

const headers: Record<string, string> = {};
if (import.meta.env.VITE_AUTH_NAME && import.meta.env.VITE_AUTH_USER) {
  headers['Remote-User'] = import.meta.env.VITE_AUTH_USER;
  headers['Remote-Name'] = import.meta.env.VITE_AUTH_NAME;
}

export const client = new ApolloClient({
  uri: import.meta.env.VITE_API_URL || `${window.origin}/api/graphql`,
  cache: new InMemoryCache(),
  headers,
});
