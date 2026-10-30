import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import { defineConfig, globalIgnores } from 'eslint/config';
import globals from 'globals';
import ts from 'typescript-eslint';

export default defineConfig(
  globalIgnores(['build/', '.svelte-kit/', 'node_modules/']),
  js.configs.recommended,
  ts.configs.recommended,
  svelte.configs.recommended,
  {
    rules: {
      // Links are external URLs or root-absolute static assets; the site has no base path.
      'svelte/no-navigation-without-resolve': 'off'
    }
  },
  {
    languageOptions: { globals: { ...globals.browser, ...globals.node } }
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts'],
    languageOptions: { parserOptions: { parser: ts.parser } }
  }
);
