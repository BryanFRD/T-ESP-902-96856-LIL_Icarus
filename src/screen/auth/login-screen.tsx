import { BrandingSection } from '@/components/auth/login/branding-section'
import { LoginSection } from '@/components/auth/login/login-section'
import { I18nToggle } from '@/components/i18n-toggle'
import { ThemeToggle } from '@/components/theme-toggle'
import { useUserContext } from '@/context/user-context.tsx'
import useRedirect from '@/hooks/common/use-redirect.ts'

export default function LoginScreen() {
  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  return (
    <div className='grid min-h-screen items-center gap-x-12 p-12 md:grid-cols-2'>
      <BrandingSection />

      <LoginSection className='w-full max-w-2xl justify-self-center' />

      <div className='absolute top-6 right-6 flex gap-x-2'>
        <I18nToggle />
        <ThemeToggle />
      </div>
    </div>
  )
}
