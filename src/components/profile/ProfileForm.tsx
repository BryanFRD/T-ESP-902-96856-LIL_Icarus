import { motion } from 'framer-motion'
import { Mail, UserIcon } from 'lucide-react'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import Button from '@/components/ui/button/Button.tsx'
import { useUserContext } from '@/context/UserContext'
import type { User } from '@/domain/types/User.ts'
import { usePatchMe } from '@/hooks/auth/usePatchMe'
import { getModifiedFields } from '@/utils/modifiedFields'

import InputField from '../ui/InputField'

const ProfileForm = () => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { mutateAsync: patchMe, isPending } = usePatchMe()
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, dirtyFields }
  } = useForm<Partial<User>>({
    defaultValues: user ?? {}
  })

  useEffect(() => {
    if (user) {
      reset(user)
    }
  }, [user, reset])

  const onSubmit = async (data: Partial<User>) => {
    const modifiedPayload = getModifiedFields<User>(data, dirtyFields)

    if (Object.keys(modifiedPayload).length > 0) {
      await toast.promise(patchMe(modifiedPayload), {
        loading: t('loading.savingChanges'),
        success: t('success.profileUpdated'),
        error: t('error.profileUpdateFailed')
      })
    }
  }

  return (
    <div className='space-y-6'>
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <h2 className='mb-2 text-2xl font-bold text-gray-800 dark:text-white'>
          {t('profileForm.title')}
        </h2>
        <p className='mb-6 text-gray-500 dark:text-gray-400'>
          {t('profileForm.description')}
        </p>
      </motion.div>

      <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
        <div className='grid grid-cols-1 gap-6 md:grid-cols-2'>
          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
          >
            <InputField
              label={t('inputField.firstName')}
              type='text'
              name='firstName'
              icon={UserIcon}
              placeholder={t('inputField.firstNamePlaceholder')}
              register={register}
              error={errors.firstName?.message}
            />
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.3 }}
          >
            <InputField
              label={t('inputField.lastName')}
              type='text'
              name='lastName'
              icon={UserIcon}
              placeholder={t('inputField.lastNamePlaceholder')}
              register={register}
              error={errors.lastName?.message}
            />
          </motion.div>
        </div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.4 }}
        >
          <InputField
            label={t('inputField.username')}
            type='text'
            name='username'
            icon={UserIcon}
            placeholder={t('inputField.usernamePlaceholder')}
            register={register}
            error={errors.username?.message}
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5 }}
        >
          <InputField
            label={t('inputField.email')}
            type='email'
            name='email'
            icon={Mail}
            placeholder={t('inputField.emailPlaceholder')}
            register={register}
            error={errors.email?.message}
          />
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.6 }}
          className='flex justify-end pt-4'
        >
          <Button
            type='submit'
            isLoading={isPending}
            loadingText={t('inputField.savingChanges')}
            className='w-full md:w-auto'
          >
            {t('inputField.saveChanges')}
          </Button>
        </motion.div>
      </form>
    </div>
  )
}

export default ProfileForm
