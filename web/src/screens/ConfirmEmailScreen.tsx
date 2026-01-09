import { CheckCircle, Loader2, XCircle } from 'lucide-react'
import { useEffect, useState } from 'react'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'

import { useConfirmEmail } from '@/hooks/auths/useConfirmEmail.ts'
import { ROUTES, getRoutePath } from '@/routes/routesName.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'
import { isNull, validToken } from '@/utils/stringUtils.ts'

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
      setErrorMessage(t('error.invalidToken'))
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
    <main className='flex min-h-screen items-center justify-center bg-gray-50 px-4 py-8 dark:bg-gray-900'>
      <div className='w-full max-w-md'>
        <div className='rounded-lg bg-white p-8 shadow-md dark:bg-gray-800'>
          <div className='text-center'>
            {status === 'loading' && (
              <>
                <div className='mb-6 flex justify-center'>
                  <Loader2 className='text-primary-600 dark:text-primary-400 h-16 w-16 animate-spin' />
                </div>
                <h1 className='mb-2 text-2xl font-bold text-gray-900 dark:text-white'>
                  {t('confirmEmail.verifying')}
                </h1>
                <p className='text-sm text-gray-600 dark:text-gray-400'>
                  {t('confirmEmail.pleaseWait')}
                </p>
              </>
            )}

            {status === 'success' && (
              <>
                <div className='mb-6 flex justify-center'>
                  <CheckCircle className='h-16 w-16 text-green-600 dark:text-green-400' />
                </div>
                <h1 className='mb-2 text-2xl font-bold text-gray-900 dark:text-white'>
                  {t('confirmEmail.success')}
                </h1>
                <p className='mb-2 text-sm text-gray-600 dark:text-gray-400'>
                  {t('confirmEmail.successMessage')}
                </p>
              </>
            )}

            {status === 'error' && (
              <>
                <div className='mb-6 flex justify-center'>
                  <XCircle className='h-16 w-16 text-red-600 dark:text-red-400' />
                </div>
                <h1 className='mb-2 text-2xl font-bold text-gray-900 dark:text-white'>
                  {t('confirmEmail.error')}
                </h1>
                <p className='mb-2 text-sm text-gray-600 dark:text-gray-400'>
                  {errorMessage || t('confirmEmail.errorMessage')}
                </p>
              </>
            )}
          </div>
        </div>
      </div>
    </main>
  )
}

export default ConfirmEmailScreen
