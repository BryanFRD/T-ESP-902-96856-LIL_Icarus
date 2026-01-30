/// <reference types="vite/client" />

// ---------------------------------------------------------
// 1. GLOBALS
// ---------------------------------------------------------
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

declare module '*.png' {
  const value: string
  export default value
}

declare module '*.jpg' {
  const value: string
  export default value
}

declare module '*.jpeg' {
  const value: string
  export default value
}

declare module '*.webp' {
  const value: string
  export default value
}

declare module '*.svg' {
  import * as React from 'react'

  export const ReactComponent: React.FunctionComponent<
    React.SVGProps<SVGSVGElement> & { title?: string }
  >

  const src: string
  export default src
}

declare module '*.css' {
  const classes: { [key: string]: string }
  export default classes
}

declare module '*.scss' {
  const classes: { [key: string]: string }
  export default classes
}
