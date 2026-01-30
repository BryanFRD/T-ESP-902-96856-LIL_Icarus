import { zodResolver } from '@hookform/resolvers/zod'
import { KeyRound } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'

import InputField from '@/components/ui/InputField.tsx'
import Button from '@/components/ui/button/Button.tsx'
import { useUserContext } from '@/context/UserContext.tsx'
import { getNewPasswordSchema } from '@/domain/validation/resetPasswordSchema.ts'
import { useResetPassword } from '@/hooks/auth/useSendPasswordResetRequest.ts'
import useRedirect from '@/hooks/common/useRedirect.ts'
import ROUTES, { getRoutePath } from '@/router/routesName.ts'
import type { ApiError } from '@/utils/apiUtils.ts'
import { validToken } from '@/utils/stringUtils.ts'

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
      toast.error(t('error.validToken'))
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
          loading: t('loading.resetPassword'),
          success: t('success.resetPassword'),
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
              {t('newPassword.title')}
            </h1>
            <p className='mt-2 text-sm text-gray-600 dark:text-gray-400'>
              {t('newPassword.subtitle')}
            </p>
          </header>

          {tokenIsValid ? (
            <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
              <InputField
                label={t('newPassword.label')}
                type='password'
                name='newPassword'
                icon={KeyRound}
                placeholder={t('newPassword.newPasswordPlaceholder')}
                register={register}
                error={errors.newPassword?.message}
                autoComplete='new-password'
              />

              <div className='flex flex-col gap-3'>
                <Button
                  type='submit'
                  variant='primary'
                  isLoading={isLoading}
                  loadingText={t('loading.resetPassword')}
                  className='w-full'
                >
                  {t('newPassword.submit')}
                </Button>

                <Button
                  type='button'
                  variant='ghost'
                  onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
                  className='w-full'
                >
                  {t('actions.cancel')}
                </Button>
              </div>
            </form>
          ) : (
            <div className='text-center'>
              <div className='mb-6 rounded-md bg-red-50 p-4 dark:bg-red-900/20'>
                <p className='text-sm font-medium text-red-700 dark:text-red-300'>
                  {t('error.validToken')}
                </p>
              </div>

              <Button
                type='button'
                onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
                variant='primary'
                className='w-full'
              >
                {t('actions.goLogin')}
              </Button>
            </div>
          )}
        </div>
      </div>
    </main>
  )
}

export default ResetPasswordScreen
