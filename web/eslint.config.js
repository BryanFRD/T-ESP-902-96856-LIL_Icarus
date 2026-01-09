// For more info, see https://github.com/storybookjs/eslint-plugin-storybook#configuration-flat-config-format
import eslint from '@eslint/js'
import pluginReact from 'eslint-plugin-react'
import globals from 'globals'
import tseslint from 'typescript-eslint'

const config = tseslint.config(
  { ignores: ['public/**', 'dist/**', 'scripts/**', 'coverage/**'] },
  { files: ['**/*.{ts,tsx}'] },
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node }
    }
  },
  eslint.configs.recommended,
  tseslint.configs.recommended,
  pluginReact.configs.flat.recommended,
  {
    files: ['**/*.{jsx,tsx}'],
    rules: {
      'no-console': 'warn',
      'react/react-in-jsx-scope': 'off',
      quotes: 'off'
    }
  }
)

export default config
