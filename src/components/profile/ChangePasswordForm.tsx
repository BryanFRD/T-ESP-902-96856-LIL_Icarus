import { motion } from 'framer-motion'
import { KeyRound } from 'lucide-react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField'
import Button from '@/components/ui/button/Button'
import { useChangePassword } from '@/hooks/user/useChangePassword'

interface ChangePasswordFormData {
  password: string
  newPassword: string
  confirmPassword: string
}

const ChangePasswordForm = () => {
  const { t } = useTranslation()
  const { mutateAsync: changePassword, isPending } = useChangePassword()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<ChangePasswordFormData>()

  const onSubmit = async (data: ChangePasswordFormData) => {
    if (data.newPassword !== data.confirmPassword) {
      toast.error(t('messages:error.passwordsDoNotMatch'))
      return
    }

    await toast.promise(changePassword(data), {
      loading: t('messages:loading.changingPassword'),
      success: t('messages:success.passwordChanged'),
      error: t('messages:error.passwordChangeFailed')
    })

    reset()
  }

  return (
    <div className='space-y-6'>
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <h2 className='mb-2 text-2xl font-bold text-gray-800 dark:text-white'>
          {t('auth:password.changetitle')}
        </h2>
        <p className='mb-6 text-gray-500 dark:text-gray-400'>
          {t('auth:password.changedescription')}
        </p>
      </motion.div>

      <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
        >
          <InputField
            label={t('auth:fields.currentPassword')}
            type='password'
            name='password'
            icon={KeyRound}
            placeholder={t('auth:fields.currentPasswordPlaceholder')}
            register={register}
            error={errors.password?.message}
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.3 }}
        >
          <InputField
            label={t('auth:fields.newPassword')}
            type='password'
            name='newPassword'
            icon={KeyRound}
            placeholder={t('auth:fields.newPasswordPlaceholder')}
            register={register}
            error={errors.newPassword?.message}
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
        >
          <InputField
            label={t('auth:fields.confirmPassword')}
            type='password'
            name='confirmPassword'
            icon={KeyRound}
            placeholder={t('auth:fields.confirmPasswordPlaceholder')}
            register={register}
            error={errors.confirmPassword?.message}
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
          className='flex justify-end pt-4'
        >
          <Button
            type='submit'
            isLoading={isPending}
            loadingText={t('auth:actions.changingPassword')}
            className='w-full md:w-auto'
          >
            {t('auth:actions.changePassword')}
          </Button>
        </motion.div>
      </form>
    </div>
  )
}

export default ChangePasswordForm
