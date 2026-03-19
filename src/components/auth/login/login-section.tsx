import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

import { LoginForm } from './login-form'

export function LoginSection({
  className,
  ...props
}: React.ComponentProps<'div'>) {
  const { t } = useTranslation('auth')

  return (
    <div className={cn('space-y-4', className)} {...props}>
      <div className='flex flex-col items-center gap-y-2'>
        <h1 className='text-3xl font-bold'>{t('login.title')}</h1>
        <h2 className='text-lg'>{t('login.subtitle')}</h2>
      </div>

      <LoginForm />

      <div className='space-x-1 text-center text-sm'>
        <span>{t('login.register.prompt')}</span>
        <Button size='none' variant='link' asChild>
          <Link to='/register'>{t('login.register.link')}</Link>
        </Button>
      </div>
    </div>
  )
}
