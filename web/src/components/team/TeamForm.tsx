import { zodResolver } from '@hookform/resolvers/zod'
import { motion } from 'framer-motion'
import { FileText, Users } from 'lucide-react'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField'
import Button from '@/components/ui/button/Button'
import type { Team } from '@/domain/types/Team.ts'
import {
  type TeamSchemaType,
  getTeamSchema
} from '@/domain/validation/teamSchema.ts'
import { useCreateTeam } from '@/hooks/team/useCreateTeam.ts'
import { usePatchTeam } from '@/hooks/team/usePatchTeam.ts'

interface TeamFormProps {
  team?: Team
  onSuccess?: (team: Team) => void
  onCancel?: () => void
}

const TeamForm = ({ team, onSuccess, onCancel }: TeamFormProps) => {
  const { t } = useTranslation()
  const { mutate: createTeam, isPending: isCreating } = useCreateTeam()
  const { mutate: patchTeam, isPending: isPatching } = usePatchTeam()

  const isEditing = !!team
  const isPending = isCreating || isPatching

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<TeamSchemaType>({
    resolver: zodResolver(getTeamSchema(t)),
    defaultValues: team
      ? { name: team.name, description: team.description || '' }
      : { name: '', description: '' }
  })

  useEffect(() => {
    if (team) {
      reset({ name: team.name, description: team.description || '' })
    }
  }, [team, reset])

  const onSubmit = (data: TeamSchemaType) => {
    if (isEditing && team) {
      toast.promise(
        new Promise<Team>((resolve, reject) => {
          patchTeam(
            { teamId: team.id, payload: data },
            {
              onSuccess: updatedTeam => {
                onSuccess?.(updatedTeam)
                resolve(updatedTeam)
              },
              onError: error => reject(error)
            }
          )
        }),
        {
          loading: t('loading.savingChanges'),
          success: t('success.teamUpdated'),
          error: t('error.teamUpdateFailed')
        }
      )
    } else {
      toast.promise(
        new Promise<Team>((resolve, reject) => {
          createTeam(data, {
            onSuccess: newTeam => {
              reset()
              onSuccess?.(newTeam)
              resolve(newTeam)
            },
            onError: error => reject(error)
          })
        }),
        {
          loading: t('loading.creatingTeam'),
          success: t('success.teamCreated'),
          error: t('error.teamCreationFailed')
        }
      )
    }
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className='space-y-6'>
      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
      >
        <InputField
          label={t('team.teamName')}
          type='text'
          name='name'
          icon={Users}
          placeholder={t('team.teamNamePlaceholder')}
          register={register}
          error={errors.name?.message}
        />
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
      >
        <div className='space-y-2'>
          <label className='flex items-center gap-2 text-sm font-medium text-gray-700 transition-all duration-300 dark:text-gray-200'>
            <FileText className='size-4' />
            {t('team.description')}
          </label>
          <textarea
            {...register('description')}
            className={`w-full rounded-xl border-2 bg-white px-4 py-3 transition-all duration-300 focus:ring-4 focus:ring-blue-100 focus:outline-none dark:bg-gray-900 dark:focus:ring-blue-900 ${
              errors.description
                ? 'border-red-300 focus:border-red-500 dark:border-red-500'
                : 'border-gray-200 focus:border-blue-500 dark:border-gray-700 dark:focus:border-blue-400'
            } text-gray-900 placeholder-gray-400 dark:text-gray-100 dark:placeholder-gray-500`}
            placeholder={t('team.descriptionPlaceholder')}
            rows={3}
          />
          {errors.description && (
            <p className='text-sm text-red-600 dark:text-red-400'>
              {errors.description.message}
            </p>
          )}
        </div>
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
          {isEditing ? t('actions.save') : t('team.createTeam')}
        </Button>
      </motion.div>
    </form>
  )
}

export default TeamForm
