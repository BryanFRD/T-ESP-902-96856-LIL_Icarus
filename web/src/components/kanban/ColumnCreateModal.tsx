import { zodResolver } from '@hookform/resolvers/zod'
import { Columns } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import InputField from '@/components/ui/InputField'
import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'
import {
  type CreateColumnSchemaType,
  createColumnSchema
} from '@/domain/validation/kanbanSchema'
import { useCreateColumn } from '@/hooks/kanban/useColumnMutations'

type ColumnCreateModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
}

const ColumnCreateModal = ({
  isOpen,
  onClose,
  auditId
}: ColumnCreateModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: createColumn, isPending } = useCreateColumn(auditId)

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors }
  } = useForm<CreateColumnSchemaType>({
    resolver: zodResolver(createColumnSchema(t)),
    defaultValues: {
      name: ''
    }
  })

  const onSubmit = async (data: CreateColumnSchemaType) => {
    try {
      await createColumn(data)
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

  return (
    <Modal
      isOpen={isOpen}
      onClose={handleClose}
      title={t('kanban:column.create')}
      size='md'
    >
      <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
        <InputField
          type='text'
          name='name'
          icon={Columns}
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
            {t('kanban:column.create')}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default ColumnCreateModal
