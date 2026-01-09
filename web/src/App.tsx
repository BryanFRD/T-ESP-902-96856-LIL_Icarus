import { Toaster } from 'react-hot-toast'

import { ThemeProvider } from './context/ThemeContext'
import { UserContextProvider } from './context/UserContext'
import Router from './routes/Router'

function App() {
  return (
    <ThemeProvider>
      <UserContextProvider>
        <Router />
        <Toaster position={'bottom-right'} />
      </UserContextProvider>
    </ThemeProvider>
  )
}

export default App
