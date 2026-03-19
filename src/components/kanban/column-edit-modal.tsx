import { zodResolver } from '@hookform/resolvers/zod'
import { Columns } from 'lucide-react'
import { useEffect } from 'react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/oldUi/InputField'
import Button from '@/components/oldUi/button/Button'
import Modal from '@/components/oldUi/modal/Modal'
import type { ColumnResponse } from '@/domain/types/kanban.ts'
import {
  type UpdateColumnSchemaType,
  updateColumnSchema
} from '@/domain/validation/kanban-schema.ts'
import { useUpdateColumn } from '@/hooks/kanban/use-column-mutation.ts'

type ColumnEditModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
  column: ColumnResponse | null
}

const ColumnEditModal = ({
  isOpen,
  onClose,
  auditId,
  column
}: ColumnEditModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: updateColumn, isPending } = useUpdateColumn(
    auditId,
    column?.id || ''
  )

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<UpdateColumnSchemaType>({
    resolver: zodResolver(updateColumnSchema(t)),
    defaultValues: {
      name: ''
    }
  })

  useEffect(() => {
    if (column) {
      reset({
        name: column.name
      })
    }
  }, [column, reset])

  const onSubmit = async (data: UpdateColumnSchemaType) => {
    try {
      await updateColumn(data)
      reset()
      onClose()
    } catch {
      // Error handled by mutation
    }
  }

  const handleClose = () => {
    reset()
    onClose()
  }

  if (!column) return null

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      title={t('kanban:column.edit')}
      size='md'
    >
      <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
        <InputField
          type='text'
          icon={Columns}
          name='name'
          register={register}
          label={t('kanban:column.name')}
          placeholder={t('kanban:column.namePlaceholder')}
          error={errors.name?.message}
        />

        <div className='flex justify-end gap-3 pt-4'>
          <Button
            type='button'
            variant='ghost'
            onClick={handleClose}
            disabled={isPending}
          >
            {t('common:cancel')}
          </Button>
          <Button type='submit' variant='primary' isLoading={isPending}>
            {t('common:save')}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default ColumnEditModal
