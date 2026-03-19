import { ThemeProvider } from '@/components/theme-provider'
import { UserContextProvider } from '@/context/user-context.tsx'
import '@/i18n'
import Router from '@/router/router.tsx'

import { Toaster } from './components/ui/sonner'
import { TooltipProvider } from './components/ui/tooltip'

function App() {
  return (
    <ThemeProvider>
      <UserContextProvider>
        <TooltipProvider>
          <Router />
          <Toaster />
        </TooltipProvider>
      </UserContextProvider>
    </ThemeProvider>
  )
}

export default App
