import BrandingSide from '@/components/auth/login/BrandingSide.tsx'
import LoginForm from '@/components/auth/login/LoginForm.tsx'
import PreferencesToggle from '@/components/ui/PreferencesToggle.tsx'
import { useUserContext } from '@/context/UserContext.tsx'
import useRedirect from '@/hooks/common/useRedirect.ts'

const LoginScreen = () => {
  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  return (
    <div className='flex min-h-screen bg-gradient-to-br from-blue-50 via-white to-blue-50 transition-all duration-300 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900'>
      <BrandingSide />

      <div className='flex flex-1 items-center justify-center p-8'>
        <div className='absolute top-6 right-6 flex gap-2'>
          <PreferencesToggle />
        </div>
        <div className='w-full max-w-md'>
          <LoginForm />
        </div>
      </div>
    </div>
  )
}

export default LoginScreen
