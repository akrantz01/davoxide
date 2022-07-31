import * as path from 'node:path';

import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@components': path.join(__dirname, 'src/components'),
      '@lib': path.join(__dirname, 'src/lib'),
    },
  },
});
