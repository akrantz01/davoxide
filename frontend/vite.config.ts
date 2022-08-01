import * as path from 'node:path';

import react from '@vitejs/plugin-react';
import { defineConfig, splitVendorChunkPlugin } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), splitVendorChunkPlugin()],
  resolve: {
    alias: {
      '@components': path.join(__dirname, 'src/components'),
      '@lib': path.join(__dirname, 'src/lib'),
    },
  },
  css: {
    modules: {
      localsConvention: 'camelCaseOnly',
    },
  },
  build: {
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          icons: ['@blueprintjs/icons'],
          apollo: ['@apollo/client'],
        },
      },
    },
  },
});
