import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { RegisterHeader } from '@/components/auth/register/register-header'
import { RegisterStepperForm } from '@/components/auth/register/register-stepper-form'
import { I18nToggle } from '@/components/i18n-toggle'
import { ThemeToggle } from '@/components/theme-toggle'
import { Button } from '@/components/ui/button'
import { useUserContext } from '@/context/user-context.tsx'
import useRedirect from '@/hooks/common/use-redirect.ts'

export default function RegisterScreen() {
  const { t } = useTranslation('auth')

  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  return (
    <div className='relative'>
      <div className='mx-auto min-h-screen w-full max-w-4xl space-y-10 p-12'>
        <RegisterHeader />

        <RegisterStepperForm />

        <div className='space-x-1 text-center text-sm'>
          <span>{t('register.login.prompt')}</span>
          <Button size='none' variant='link' asChild>
            <Link to='/login'>{t('register.login.link')}</Link>
          </Button>
        </div>

        <div className='absolute top-6 right-6 flex gap-x-2'>
          <I18nToggle />
          <ThemeToggle />
        </div>
      </div>
    </div>
  )
}
