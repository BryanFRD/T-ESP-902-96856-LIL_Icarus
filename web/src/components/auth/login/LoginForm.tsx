import { zodResolver } from '@hookform/resolvers/zod'
import { Lock, Mail } from 'lucide-react'
import { useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import LoginHeader from '@/components/auth/login/LoginHeader.tsx'
import ModalSendResetPassword from '@/components/auth/reset-password/ModalSendResetPassword.tsx'
import InputField from '@/components/ui/InputField.tsx'
import { useUserContext } from '@/context/UserContext.tsx'
import {
  type LoginSchemaType,
  getLoginSchema
} from '@/domain/validation/loginSchema.ts'
import useToggle from '@/hooks/common/useToggle.ts'
import type { ApiError } from '@/utils/apiUtils.ts'

import Button from '../../ui/button/Button.tsx'

const LoginForm = () => {
  const { t } = useTranslation()
  const { login } = useUserContext()
  const { value, toggle, close } = useToggle()
  const [isLoading, setIsLoading] = useState(false)
  const [apiError, setApiError] = useState<string | null>(null)

  const {
    register,
    handleSubmit,
    formState: { errors }
  } = useForm({
    resolver: zodResolver(getLoginSchema(t))
  })

  const onSubmit = async (data: LoginSchemaType) => {
    setIsLoading(true)
    setApiError(null)
    try {
      await toast.promise(login(data), {
        loading: t('loading.login'),
        success: () => t('success.login'),
        error: (err: ApiError) => t(err.errorKey)
      })
    } finally {
      setIsLoading(false)
    }
  }

  return (
    <div className='mx-auto w-full max-w-md'>
      <LoginHeader />

      <form
        onSubmit={handleSubmit(onSubmit)}
        className='flex flex-col space-y-6 rounded-lg border border-gray-200 bg-white p-8 transition-all duration-300 dark:border-none dark:bg-gray-900'
      >
        <InputField
          label={t('inputField.email')}
          type='email'
          name='email'
          icon={Mail}
          placeholder={t('inputField.emailPlaceholder')}
          register={register}
          error={errors.email?.message}
          autoComplete='email'
        />

        <InputField
          label={t('inputField.password')}
          type='password'
          name='password'
          icon={Lock}
          placeholder={t('inputField.passwordPlaceholder')}
          register={register}
          error={errors.password?.message}
          autoComplete='current-password'
        />

        {apiError && (
          <div className='text-center text-sm text-red-600 dark:text-red-400'>
            {apiError}
          </div>
        )}

        <Button
          type={undefined}
          onClick={toggle}
          size={'sm'}
          variant={'text'}
          className={'self-end'}
          withBorder={false}
        >
          {t('inputField.forgotPassword')}
        </Button>

        <Button
          type='submit'
          isLoading={isLoading}
          loadingText={t('inputField.signingIn')}
          className='w-full bg-orange-600 text-white hover:bg-orange-700 dark:bg-orange-500 dark:text-white dark:hover:bg-orange-600'
        >
          {t('inputField.signIn')}
        </Button>
      </form>

      <div className='mt-8 text-center'>
        <p className='flex items-center justify-center gap-1 text-gray-600 transition-all duration-300 dark:text-gray-400'>
          {t('inputField.noAccount')}
          <Link
            to='/register'
            className='font-semibold text-orange-600 transition-colors hover:text-orange-700 dark:text-orange-400 dark:hover:text-orange-300'
          >
            {t('inputField.signUpHere')}
          </Link>
        </p>
      </div>
      <ModalSendResetPassword isOpen={value} onClose={close} />
    </div>
  )
}

export default LoginForm
