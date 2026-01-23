import tailwindcss from '@tailwindcss/vite'
import react from '@vitejs/plugin-react'
import path from 'node:path'
import { defineConfig } from 'vite'

type ENVIRONMENT = 'sandbox' | 'development' | 'production'

const API_URLS: Record<ENVIRONMENT, string> = {
  sandbox: 'http://localhost:8080',
  development: 'https://api-dev.icarus-cyber.fr',
  production: 'https://api.icarus-cyber.fr'
}

export default defineConfig(async ({ mode }) => {
  const currentMode = mode as ENVIRONMENT
  const config = {
    CURRENT_MODE: currentMode,
    ICARUS_API_URL: API_URLS[currentMode]
  }

  console.log(
    `Running in ${currentMode} mode with API URL: ${config.ICARUS_API_URL}`
  )

  return {
    plugins: [react(), tailwindcss()],
    define: { __CONFIG__: config },
    resolve: {
      alias: {
        '@': path.resolve(__dirname, './src')
      }
    },
    test: {
      globals: true,
      environment: 'happy-dom',
      setupFiles: ['src/test/setup.ts'],
      include: ['src/**/*.test.{ts,tsx}', 'src/**/*.spec.{ts,tsx}'],
      exclude: ['node_modules/', 'dist/'],
      coverage: {
        provider: 'v8',
        reporter: ['text', 'json', 'html', 'lcov'],
        include: ['src/**/*.{ts,tsx}'],
        exclude: [
          'node_modules/',
          'src/**/*.test.ts',
          'src/**/*.test.tsx',
          'src/**/*.spec.ts',
          'src/**/*.spec.tsx',
          'src/**/*.d.ts',
          'src/**/*.stories.tsx',
          'src/test/**'
        ]
      }
    }
  }
})
