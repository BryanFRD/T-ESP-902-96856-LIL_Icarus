import { zodResolver } from '@hookform/resolvers/zod'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'
import { useLocation, useNavigate } from 'react-router-dom'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import {
  Field,
  FieldError,
  FieldGroup,
  FieldLabel
} from '@/components/ui/field'
import { Input } from '@/components/ui/input'
import { useUserContext } from '@/context/user-context'
import { getNewPasswordSchema } from '@/domain/validation/reset-password-schema'
import { useResetPassword } from '@/hooks/auth/use-send-password-reset-request'
import useRedirect from '@/hooks/common/use-redirect'
import ROUTES, { getRoutePath } from '@/router/routes-name'
import { validToken } from '@/utils/string-utils'

type FormValues = { newPassword: string }

const ResetPasswordScreen = () => {
  const { isAuthenticated } = useUserContext()

  useRedirect({ when: isAuthenticated })

  const { t } = useTranslation()
  const location = useLocation()
  const navigate = useNavigate()
  const { mutateAsync, isPending } = useResetPassword()

  const form = useForm<FormValues>({
    resolver: zodResolver(getNewPasswordSchema(t as (key: string) => string)),
    defaultValues: { newPassword: '' }
  })

  const token = new URLSearchParams(location.search).get('token')
  const tokenIsValid = validToken(token)

  const onSubmit = async (data: FormValues) => {
    if (!tokenIsValid) {
      toast.error(t('messages:error.validToken'))
      return
    }

    try {
      await mutateAsync({ newPassword: data.newPassword, token: token! })
      toast.success(t('messages:success.resetPassword'))
      navigate(getRoutePath(ROUTES.LOGIN))
    } catch {
      toast.error(t('messages:error.resetPasswordFailed'))
    }
  }

  return (
    <main className='flex min-h-screen items-center justify-center bg-gray-50 px-4 py-8 dark:bg-gray-900'>
      <div className='w-full max-w-md'>
        <div className='rounded-lg bg-white p-8 shadow-md dark:bg-gray-800'>
          <header className='mb-8 text-center'>
            <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
              {t('auth:password.resetTitle')}
            </h1>
            <p className='mt-2 text-sm text-gray-600 dark:text-gray-400'>
              {t('auth:password.resetSubtitle')}
            </p>
          </header>

          {tokenIsValid ? (
            <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-6'>
              <FieldGroup>
                <Controller
                  control={form.control}
                  name='newPassword'
                  render={({ field, fieldState }) => (
                    <Field data-invalid={fieldState.invalid}>
                      <FieldLabel htmlFor={field.name}>
                        {t('auth:password.resetLabel')}
                      </FieldLabel>
                      <Input
                        {...field}
                        id={field.name}
                        type='password'
                        aria-invalid={fieldState.invalid}
                        placeholder={t('auth:password.resetPlaceholder')}
                        autoComplete='new-password'
                      />
                      {fieldState.invalid && (
                        <FieldError errors={[fieldState.error]} />
                      )}
                    </Field>
                  )}
                />
              </FieldGroup>

              <div className='flex flex-col gap-3'>
                <Button type='submit' className='w-full' disabled={isPending}>
                  {t('auth:password.resetSubmit')}
                </Button>

                <Button
                  type='button'
                  variant='outline'
                  className='w-full'
                  onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
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
                className='w-full'
                onClick={() => navigate(getRoutePath(ROUTES.LOGIN))}
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
