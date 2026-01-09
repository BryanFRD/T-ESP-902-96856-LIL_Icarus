import { FlatCompat } from '@eslint/eslintrc'
import eslint from '@eslint/js'
import pluginReact from 'eslint-plugin-react'
import eslintPluginUnicorn from 'eslint-plugin-unicorn'
import globals from 'globals'
import tseslint from 'typescript-eslint'

const compat = new FlatCompat({
  baseDirectory: import.meta.dirname
})

const config = tseslint.config(
  { ignores: ['.next/**', 'public/**'] },
  { files: ['**/*.{ts,tsx}'] },
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node }
    }
  },
  eslint.configs.recommended,
  tseslint.configs.recommended,
  pluginReact.configs.flat.recommended,
  eslintPluginUnicorn.configs.recommended,
  ...compat.extends('next/core-web-vitals', 'next/typescript'),
  ...compat.plugins('only-warn'),
  {
    rules: {
      'react/react-in-jsx-scope': 'off',
      'tailwindcss/no-custom-classname': 'off',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          argsIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_'
        }
      ],
      'unicorn/prevent-abbreviations': 'off',
      'unicorn/no-nested-ternary': 'off',
      'react-hooks/exhaustive-deps': 'off',
      'unicorn/prefer-string-raw': 'off'
    }
  },
  {
    files: ['**/*.{jsx,tsx}'],
    rules: {
      'no-console': 'warn'
    }
  }
)

export default config
