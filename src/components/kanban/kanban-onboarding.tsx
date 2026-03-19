import { zodResolver } from '@hookform/resolvers/zod'
import { ArrowRight, Plus, Trash2 } from 'lucide-react'
import { useState } from 'react'
import { useFieldArray, useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'
import { z } from 'zod'

import Button from '@/components/oldUi/button/Button'

type ColumnFormData = {
  columns: Array<{ name: string }>
}

type KanbanOnboardingProps = {
  auditId: string
  onComplete: () => void
}

const KanbanOnboarding = ({ auditId, onComplete }: KanbanOnboardingProps) => {
  const { t } = useTranslation()
  const [isSubmitting, setIsSubmitting] = useState(false)

  const columnSchema = z.object({
    columns: z
      .array(
        z.object({
          name: z
            .string()
            .min(1, t('kanban:validation.column.name.required'))
            .max(50, t('kanban:validation.column.name.max'))
        })
      )
      .min(1, t('kanban:onboarding.validation.minOneColumn'))
  })

  const {
    register,
    control,
    handleSubmit,
    formState: { errors }
  } = useForm<ColumnFormData>({
    resolver: zodResolver(columnSchema),
    defaultValues: {
      columns: [
        { name: t('kanban:onboarding.defaultColumns.todo') },
        { name: t('kanban:onboarding.defaultColumns.inProgress') },
        { name: t('kanban:onboarding.defaultColumns.review') },
        { name: t('kanban:onboarding.defaultColumns.done') }
      ]
    }
  })

  const { fields, append, remove } = useFieldArray({
    control,
    name: 'columns'
  })

  const onSubmit = async (data: ColumnFormData) => {
    setIsSubmitting(true)
    try {
      const { createColumn } = await import('@/service/kanban-service.ts')

      for (const column of data.columns) {
        await createColumn(auditId, { name: column.name })
      }

      toast.success(t('kanban:onboarding.success'))
      onComplete()
    } catch {
      toast.error(t('kanban:onboarding.error'))
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className='flex h-full w-full items-center justify-center p-8'>
      <div className='w-full max-w-3xl'>
        <div className='mb-12'>
          <div className='mb-3 flex items-center gap-3'>
            <div>
              <h1 className='text-3xl font-bold text-gray-900 dark:text-gray-100'>
                {t('kanban:onboarding.title')}
              </h1>
              <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
                {t('kanban:onboarding.subtitle')}
              </p>
            </div>
          </div>
        </div>

        <form onSubmit={handleSubmit(onSubmit)} className='space-y-8'>
          <div className='space-y-4'>
            <div className='flex items-center justify-between'>
              <div>
                <h2 className='text-lg font-semibold text-gray-900 dark:text-gray-100'>
                  {t('kanban:onboarding.columnsTitle')}
                </h2>
                <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
                  {t('kanban:onboarding.columnsDescription')}
                </p>
              </div>
              <Button
                type='button'
                variant='outline'
                size='sm'
                onClick={() => append({ name: '' })}
                disabled={isSubmitting}
              >
                <Plus className='mr-2 h-4 w-4' />
                {t('kanban:onboarding.addColumn')}
              </Button>
            </div>

            <div className='grid gap-3'>
              {fields.map((field, index) => (
                <div
                  key={field.id}
                  className='group relative rounded-lg border-2 border-gray-200 bg-white p-4 transition-all hover:border-orange-200 dark:border-gray-700 dark:bg-gray-800 dark:hover:border-orange-900/50'
                >
                  <div className='flex items-center gap-3'>
                    <div className='flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-lg bg-gray-100 text-sm font-semibold text-gray-600 dark:bg-gray-700 dark:text-gray-300'>
                      {index + 1}
                    </div>
                    <div className='flex-1'>
                      <input
                        {...register(`columns.${index}.name`)}
                        type='text'
                        placeholder={t('kanban:onboarding.columnPlaceholder')}
                        className='w-full border-0 bg-transparent px-0 py-0 text-base font-medium text-gray-900 outline-none placeholder:text-gray-400 dark:text-gray-100 dark:placeholder:text-gray-500'
                        disabled={isSubmitting}
                      />
                      {errors.columns?.[index]?.name && (
                        <p className='mt-1 text-sm text-red-600 dark:text-red-400'>
                          {errors.columns[index]?.name?.message}
                        </p>
                      )}
                    </div>
                    {fields.length > 1 && (
                      <button
                        type='button'
                        onClick={() => remove(index)}
                        disabled={isSubmitting}
                        className='flex h-8 w-8 items-center justify-center rounded-lg text-gray-400 opacity-0 transition-all group-hover:opacity-100 hover:bg-red-50 hover:text-red-600 disabled:opacity-50 dark:hover:bg-red-900/20 dark:hover:text-red-400'
                        title={t('kanban:onboarding.removeColumn')}
                      >
                        <Trash2 className='h-4 w-4' />
                      </button>
                    )}
                  </div>
                </div>
              ))}
            </div>

            {errors.columns?.root && (
              <p className='text-sm text-red-600 dark:text-red-400'>
                {errors.columns.root.message}
              </p>
            )}
          </div>

          <div className='flex items-start gap-3 rounded-xl border-2 border-blue-100 bg-blue-50 p-4 dark:border-blue-900/30 dark:bg-blue-900/10'>
            <div className='flex h-6 w-6 flex-shrink-0 items-center justify-center'>
              <svg
                className='h-5 w-5 text-blue-600 dark:text-blue-400'
                fill='none'
                viewBox='0 0 24 24'
                stroke='currentColor'
              >
                <path
                  strokeLinecap='round'
                  strokeLinejoin='round'
                  strokeWidth={2}
                  d='M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z'
                />
              </svg>
            </div>
            <p className='text-sm leading-relaxed text-blue-900 dark:text-blue-200'>
              {t('kanban:onboarding.tip')}
            </p>
          </div>

          <div className='flex justify-end gap-3 border-t border-gray-200 pt-6 dark:border-gray-700'>
            <Button
              type='submit'
              variant='primary'
              size='lg'
              isLoading={isSubmitting}
              disabled={isSubmitting || fields.length === 0}
            >
              {t('kanban:onboarding.createBoard')}
              <ArrowRight className='ml-2 h-5 w-5' />
            </Button>
          </div>
        </form>
      </div>
    </div>
  )
}

export default KanbanOnboarding
