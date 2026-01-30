import { zodResolver } from '@hookform/resolvers/zod'
import { motion } from 'framer-motion'
import { Calendar, Users } from 'lucide-react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField'
import Button from '@/components/ui/button/Button'
import type { Invitation } from '@/domain/types/Team.ts'
import {
  type InvitationSchemaType,
  getInvitationSchema
} from '@/domain/validation/invitationSchema.ts'
import { useCreateInvitation } from '@/hooks/team/useCreateInvitation.ts'

interface InvitationFormProps {
  teamId: string
  onSuccess?: (invitation: Invitation) => void
  onCancel?: () => void
}

const InvitationForm = ({
  teamId,
  onSuccess,
  onCancel
}: InvitationFormProps) => {
  const { t } = useTranslation()
  const { mutate: createInvitation, isPending } = useCreateInvitation()

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<InvitationSchemaType>({
    resolver: zodResolver(getInvitationSchema(t)),
    defaultValues: {
      maxUses: 1,
      expiresAt: ''
    }
  })

  const onSubmit = (data: InvitationSchemaType) => {
    const payload: { maxUses: number; expiresAt?: string } = {
      maxUses: data.maxUses
    }

    if (data.expiresAt) {
      payload.expiresAt = new Date(data.expiresAt).toISOString()
    }

    toast.promise(
      new Promise<Invitation>((resolve, reject) => {
        createInvitation(
          { teamId, payload },
          {
            onSuccess: newInvitation => {
              reset()
              onSuccess?.(newInvitation)
              resolve(newInvitation)
            },
            onError: error => reject(error)
          }
        )
      }),
      {
        loading: t('loading.creatingInvitation'),
        success: t('success.invitationCreated'),
        error: t('error.invitationCreationFailed')
      }
    )
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <InputField
          label={t('team.maxUses')}
          type='number'
          name='maxUses'
          icon={Users}
          placeholder={t('team.maxUsesPlaceholder')}
          register={register}
          error={errors.maxUses?.message}
          registerOptions={{ valueAsNumber: true }}
        />
        <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
          {t('team.maxUsesHint')}
        </p>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
      >
        <InputField
          label={t('team.expiresAt')}
          type='datetime-local'
          name='expiresAt'
          icon={Calendar}
          register={register}
          error={errors.expiresAt?.message}
        />
        <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
          {t('team.expiresAtHint')}
        </p>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
        className='flex justify-end gap-3 pt-4'
      >
        {onCancel && (
          <Button type='button' variant='secondary' onClick={onCancel}>
            {t('actions.cancel')}
          </Button>
        )}
        <Button type='submit' isLoading={isPending}>
          {t('team.createInvitation')}
        </Button>
      </motion.div>
    </form>
  )
}

export default InvitationForm
