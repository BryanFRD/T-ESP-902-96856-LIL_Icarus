/// <reference types="vite/client" />

declare global {
  const __CONFIG__: {
    CURRENT_MODE: string
    ICARUS_API_URL: string
  }

  interface Window {
    queryClient?: import('@tanstack/react-query').QueryClient
  }

  var queryClient: import('@tanstack/react-query').QueryClient | undefined
}

export {}
