import { CheckCircle, XCircle } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'

import { BrandingSection } from '@/components/auth/login/branding-section'
import { I18nToggle } from '@/components/i18n-toggle'
import { ThemeToggle } from '@/components/theme-toggle'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader } from '@/components/ui/card'
import { Spinner } from '@/components/ui/spinner'
import { useConfirmEmail } from '@/hooks/auth/use-confirm-email.ts'
import { ROUTES, getRoutePath } from '@/router/routes-name.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'
import { isNull, validToken } from '@/utils/string-utils.ts'

type Status = 'loading' | 'success' | 'error'

const ConfirmEmailScreen = () => {
  const { mutateAsync } = useConfirmEmail()
  const { t } = useTranslation()
  const location = useLocation()
  const navigate = useNavigate()
  const [status, setStatus] = useState<Status>('loading')
  const [errorMessage, setErrorMessage] = useState<string>('')

  const token = new URLSearchParams(location.search).get('token')

  useEffect(() => {
    if (isNull(token) || !validToken(token)) {
      setStatus('error')
      setErrorMessage(t('messages:error.invalidToken'))
      return
    }

    mutateAsync(token)
      .then(() => {
        setStatus('success')
      })
      .catch((err: unknown) => {
        setStatus('error')
        const message = getErrorMessage(err, 'confirmEmail.errorMessage')
        setErrorMessage(t(message))
      })
      .finally(() => {
        setTimeout(() => {
          navigate(getRoutePath(ROUTES.LOGIN))
        }, 3000)
      })
  }, [token, mutateAsync, navigate, t])

  return (
    <div className='grid min-h-screen grid-cols-2 items-center gap-x-12 p-12'>
      <BrandingSection />

      <div className='w-full max-w-2xl justify-self-center'>
        <Card>
          <CardHeader className='items-center text-center'>
            {status === 'loading' && (
              <Spinner className='text-primary size-12' />
            )}
            {status === 'success' && (
              <CheckCircle className='size-12 text-green-500' />
            )}
            {status === 'error' && (
              <XCircle className='text-destructive size-12' />
            )}

            <h1 className='text-2xl font-bold'>
              {status === 'loading' && t('auth:email.confirmverifying')}
              {status === 'success' && t('auth:email.confirmsuccess')}
              {status === 'error' && t('auth:email.confirmerror')}
            </h1>

            <p className='text-muted-foreground text-sm'>
              {status === 'loading' && t('auth:email.confirmpleaseWait')}
              {status === 'success' && t('auth:email.confirmsuccessMessage')}
              {status === 'error' &&
                (errorMessage || t('auth:email.confirmerrorMessage'))}
            </p>
          </CardHeader>

          {status !== 'loading' && (
            <CardContent className='flex justify-center'>
              <Button onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}>
                {t('common:actions.goLogin')}
              </Button>
            </CardContent>
          )}
        </Card>
      </div>

      <div className='absolute top-6 right-6 flex gap-x-2'>
        <I18nToggle />
        <ThemeToggle />
      </div>
    </div>
  )
}

export default ConfirmEmailScreen
