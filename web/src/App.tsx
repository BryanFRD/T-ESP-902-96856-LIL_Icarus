import { X } from 'lucide-react'
import { ToastBar, Toaster, toast } from 'react-hot-toast'

import { ThemeProvider } from '@/context/ThemeContext'
import { UserContextProvider } from '@/context/UserContext'
import Router from '@/router/Router'

function App() {
  return (
    <ThemeProvider>
      <UserContextProvider>
        <Router />
        <Toaster
          position='top-center'
          gutter={12}
          containerStyle={{
            top: 40
          }}
          toastOptions={{
            className: '',
            style: {
              border: '1px solid #E0E0E0',
              padding: '12px 16px',
              color: '#333',
              fontSize: '1rem',
              borderRadius: '12px',
              maxWidth: '500px',
              boxShadow: '0px 4px 12px rgba(0, 0, 0, 0.1)'
            },
            duration: 2500,
            success: {
              style: {
                background: '#F0FDF4',
                border: '1px solid #22C55E',
                color: '#15803D'
              },
              iconTheme: {
                primary: '#22C55E',
                secondary: '#FFFAEE'
              }
            },
            error: {
              duration: 4000,
              style: {
                background: '#FEF2F2',
                border: '1px solid #EF4444',
                color: '#B91C1C'
              }
            }
          }}
        >
          {t => (
            <ToastBar toast={t}>
              {({ icon, message }) => (
                <div className='flex w-full items-center gap-2'>
                  {icon}
                  <div className='flex-1 text-sm font-medium'>{message}</div>
                  {t.type !== 'loading' && (
                    <button
                      onClick={() => toast.dismiss(t.id)}
                      className='rounded-full p-1 text-gray-400 transition-colors hover:bg-black/5 hover:text-gray-600'
                      aria-label='Fermer'
                    >
                      <X size={18} />
                    </button>
                  )}
                </div>
              )}
            </ToastBar>
          )}
        </Toaster>
      </UserContextProvider>
    </ThemeProvider>
  )
}

export default App
