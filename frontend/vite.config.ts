import { fileURLToPath, URL } from 'node:url';
import react from '@vitejs/plugin-react';
import { defineConfig } from 'vitest/config';

const root = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url)),
      '@locales': fileURLToPath(new URL('./shared/locales', import.meta.url)),
    },
  },
  build: {
    rollupOptions: {
      input: {
        pet: fileURLToPath(new URL('./pet.html', import.meta.url)),
        bubble: fileURLToPath(new URL('./bubble.html', import.meta.url)),
        settings: fileURLToPath(new URL('./settings.html', import.meta.url)),
        onboarding: fileURLToPath(new URL('./onboarding.html', import.meta.url)),
      },
    },
  },
  server: {
    port: 1420,
    strictPort: true,
  },
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: './src/test/setup.ts',
    css: false,
  },
  // Keep the resolved root explicit for tooling that reads it.
  root,
});
