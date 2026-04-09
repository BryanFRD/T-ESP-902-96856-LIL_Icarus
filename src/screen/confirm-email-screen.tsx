import { AlertCircle, CheckCircle } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'

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
        const message = getErrorMessage(
          err,
          'messages:error.confirmEmailFailed'
        )
        setErrorMessage(t(message))
      })
      .finally(() => {
        setTimeout(() => {
          navigate(getRoutePath(ROUTES.LOGIN))
        }, 3000)
      })
  }, [token, mutateAsync, navigate, t])

  return (
    <div className='flex min-h-screen w-full items-center justify-center p-12'>
      <div className='w-full max-w-2xl'>
        <Card>
          <CardHeader>
            <div className='flex flex-col items-center gap-4 text-center'>
              {status === 'loading' && (
                <Spinner className='text-primary size-12' />
              )}
              {status === 'success' && (
                <CheckCircle className='size-12 text-green-500' />
              )}
              {status === 'error' && (
                <AlertCircle className='text-destructive size-12' />
              )}

              <div className='flex flex-col gap-1'>
                <h1 className='text-2xl font-bold'>
                  {status === 'loading' && t('auth:email.confirmverifying')}
                  {status === 'success' && t('auth:email.confirmsuccess')}
                  {status === 'error' && t('auth:email.confirmerror')}
                </h1>

                <p className='text-muted-foreground text-sm'>
                  {status === 'loading' && t('auth:email.confirmpleaseWait')}
                  {status === 'success' &&
                    t('auth:email.confirmsuccessMessage')}
                  {status === 'error' &&
                    (errorMessage || t('auth:email.confirmerrorMessage'))}
                </p>
              </div>
            </div>
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
