import RegisterStepForm from '@/components/auth/register/RegisterStepForm.tsx'
import PreferencesToggle from '@/components/ui/PreferencesToggle.tsx'
import { useUserContext } from '@/context/UserContext.tsx'
import useRedirect from '@/hooks/common/useRedirect.ts'

const RegisterScreen = () => {
  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  return (
    <div className='flex min-h-screen bg-gradient-to-br from-blue-50 via-white to-blue-50 transition-all duration-300 dark:from-gray-900 dark:via-gray-950 dark:to-gray-900'>
      <div className='absolute top-6 right-6 flex gap-2'>
        <PreferencesToggle />
      </div>
      <RegisterStepForm />
    </div>
  )
}

export default RegisterScreen
