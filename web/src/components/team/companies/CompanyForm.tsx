import { zodResolver } from '@hookform/resolvers/zod'
import { motion } from 'framer-motion'
import { Building2, FileText, Hash } from 'lucide-react'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField'
import Button from '@/components/ui/button/Button'
import type { Company } from '@/domain/types/Team.ts'
import {
  type CompanySchemaType,
  getCompanySchema
} from '@/domain/validation/companySchema.ts'
import { useCreateCompany } from '@/hooks/team/useCreateCompany.ts'
import { usePatchCompany } from '@/hooks/team/usePatchCompany.ts'

interface CompanyFormProps {
  teamId: string
  company?: Company
  onSuccess?: (company: Company) => void
  onCancel?: () => void
}

const CompanyForm = ({
  teamId,
  company,
  onSuccess,
  onCancel
}: CompanyFormProps) => {
  const { t } = useTranslation()
  const { mutate: createCompany, isPending: isCreating } = useCreateCompany()
  const { mutate: patchCompany, isPending: isPatching } = usePatchCompany()

  const isEditing = !!company
  const isPending = isCreating || isPatching

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<CompanySchemaType>({
    resolver: zodResolver(getCompanySchema(t)),
    defaultValues: company
      ? {
          name: company.name,
          siret: company.siret,
          description: company.description || ''
        }
      : { name: '', siret: '', description: '' }
  })

  useEffect(() => {
    if (company) {
      reset({
        name: company.name,
        siret: company.siret,
        description: company.description || ''
      })
    }
  }, [company, reset])

  const onSubmit = (data: CompanySchemaType) => {
    const payload = {
      name: data.name,
      siret: data.siret,
      description: data.description || undefined
    }

    if (isEditing && company) {
      toast.promise(
        new Promise<Company>((resolve, reject) => {
          patchCompany(
            { teamId, companyId: company.id, payload },
            {
              onSuccess: updatedCompany => {
                onSuccess?.(updatedCompany)
                resolve(updatedCompany)
              },
              onError: error => reject(error)
            }
          )
        }),
        {
          loading: t('messages:loading.savingChanges'),
          success: t('messages:success.companyUpdated'),
          error: t('messages:error.companyUpdateFailed')
        }
      )
    } else {
      toast.promise(
        new Promise<Company>((resolve, reject) => {
          createCompany(
            { teamId, payload },
            {
              onSuccess: newCompany => {
                reset()
                onSuccess?.(newCompany)
                resolve(newCompany)
              },
              onError: error => reject(error)
            }
          )
        }),
        {
          loading: t('messages:loading.creatingCompany'),
          success: t('messages:success.companyCreated'),
          error: t('messages:error.companyCreationFailed')
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
          label={t('team:companyName')}
          type='text'
          name='name'
          icon={Building2}
          placeholder={t('team:companyNamePlaceholder')}
          register={register}
          error={errors.name?.message}
        />
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.2 }}
      >
        <InputField
          label={t('team:siret')}
          type='text'
          name='siret'
          icon={Hash}
          placeholder={t('team:siretPlaceholder')}
          register={register}
          error={errors.siret?.message}
        />
        <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
          {t('team:siretHint')}
        </p>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.3 }}
      >
        <div className='space-y-2'>
          <label className='flex items-center gap-2 text-sm font-medium text-gray-700 transition-all duration-300 dark:text-gray-200'>
            <FileText className='h-4 w-4' />
            {t('team:description')}
          </label>
          <textarea
            {...register('description')}
            className={`w-full rounded-xl border-2 bg-white px-4 py-3 transition-all duration-300 focus:ring-4 focus:ring-blue-100 focus:outline-none dark:bg-gray-900 dark:focus:ring-blue-900 ${
              errors.description
                ? 'border-red-300 focus:border-red-500 dark:border-red-500'
                : 'border-gray-200 focus:border-blue-500 dark:border-gray-700 dark:focus:border-blue-400'
            } text-gray-900 placeholder-gray-400 dark:text-gray-100 dark:placeholder-gray-500`}
            placeholder={t('team:companyDescriptionPlaceholder')}
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
        transition={{ delay: 0.4 }}
        className='flex justify-end gap-3 pt-4'
      >
        {onCancel && (
          <Button type='button' variant='secondary' onClick={onCancel}>
            {t('common:actions.cancel')}
          </Button>
        )}
        <Button type='submit' isLoading={isPending}>
          {isEditing ? t('common:actions.save') : t('team:createCompany')}
        </Button>
      </motion.div>
    </form>
  )
}

export default CompanyForm
