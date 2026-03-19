import { zodResolver } from '@hookform/resolvers/zod'
import { KeyRound } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'

import InputField from '@/components/oldUi/InputField.tsx'
import Button from '@/components/oldUi/button/Button.tsx'
import { useUserContext } from '@/context/user-context.tsx'
import { getNewPasswordSchema } from '@/domain/validation/reset-password-schema.ts'
import { useResetPassword } from '@/hooks/auth/use-send-password-reset-request.ts'
import useRedirect from '@/hooks/common/use-redirect.ts'
import ROUTES, { getRoutePath } from '@/router/routes-name.ts'
import type { ApiError } from '@/utils/api-utils.ts'
import { validToken } from '@/utils/string-utils.ts'

type FormValues = { newPassword: string }

const ResetPasswordScreen = () => {
  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  const { t } = useTranslation()
  const location = useLocation()
  const navigate = useNavigate()
  const { mutateAsync } = useResetPassword()
  const [isLoading, setIsLoading] = useState(false)

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm<FormValues>({
    resolver: zodResolver(getNewPasswordSchema(t))
  })

  const token = new URLSearchParams(location.search).get('token')
  const tokenIsValid = validToken(token)

  const onSubmit = async (data: FormValues) => {
    if (!tokenIsValid) {
      toast.error(t('messages:error.validToken'))
      return
    }

    setIsLoading(true)

    try {
      await toast.promise(
        mutateAsync({ newPassword: data.newPassword, token: token! }).then(
          () => {
            navigate(getRoutePath(ROUTES.LOGIN))
          }
        ),
        {
          loading: t('messages:loading.resetPassword'),
          success: t('messages:success.resetPassword'),
          error: (err: ApiError) => t(err.errorKey)
        }
      )
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <main className='flex min-h-screen items-center justify-center bg-gray-50 px-4 py-8 dark:bg-gray-900'>
      <div className='w-full max-w-md'>
        <div className='rounded-lg bg-white p-8 shadow-md dark:bg-gray-800'>
          <header className='mb-8 text-center'>
            <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
              {t('auth:password.resettitle')}
            </h1>
            <p className='mt-2 text-sm text-gray-600 dark:text-gray-400'>
              {t('auth:password.resetsubtitle')}
            </p>
          </header>

          {tokenIsValid ? (
            <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
              <InputField
                label={t('auth:password.resetlabel')}
                type='password'
                name='newPassword'
                icon={KeyRound}
                placeholder={t('auth:password.resetnewPasswordPlaceholder')}
                register={register}
                error={errors.newPassword?.message}
                autoComplete='new-password'
              />

              <div className='flex flex-col gap-3'>
                <Button
                  type='submit'
                  variant='primary'
                  isLoading={isLoading}
                  loadingText={t('messages:loading.resetPassword')}
                  className='w-full'
                >
                  {t('auth:password.resetsubmit')}
                </Button>

                <Button
                  type='button'
                  variant='ghost'
                  onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
                  className='w-full'
                >
                  {t('common:actions.cancel')}
                </Button>
              </div>
            </form>
          ) : (
            <div className='text-center'>
              <div className='mb-6 rounded-md bg-red-50 p-4 dark:bg-red-900/20'>
                <p className='text-sm font-medium text-red-700 dark:text-red-300'>
                  {t('messages:error.validToken')}
                </p>
              </div>

              <Button
                type='button'
                onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
                variant='primary'
                className='w-full'
              >
                {t('common:actions.goLogin')}
              </Button>
            </div>
          )}
        </div>
      </div>
    </main>
  )
}

export default ResetPasswordScreen
