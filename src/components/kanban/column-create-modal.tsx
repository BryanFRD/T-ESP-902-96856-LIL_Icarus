import { zodResolver } from '@hookform/resolvers/zod'
import { Loader2 } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle
} from '@/components/ui/dialog'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  type CreateColumnSchemaType,
  createColumnSchema
} from '@/domain/validation/kanban-schema.ts'
import { useCreateColumn } from '@/hooks/kanban/use-column-mutation.ts'

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
    defaultValues: { name: '' }
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
    <Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
      <DialogContent className='sm:max-w-md'>
        <DialogHeader>
          <DialogTitle>{t('kanban:column.create')}</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          <div className='space-y-2'>
            <Label htmlFor='col-name'>{t('kanban:column.name')}</Label>
            <Input
              id='col-name'
              placeholder={t('kanban:column.namePlaceholder')}
              {...register('name')}
            />
            {errors.name && (
              <p className='text-destructive text-xs'>{errors.name.message}</p>
            )}
          </div>

          <div className='flex justify-end gap-3 pt-2'>
            <Button
              type='button'
              variant='outline'
              onClick={handleClose}
              disabled={isPending}
            >
              {t('common:actions.cancel')}
            </Button>
            <Button type='submit' disabled={isPending}>
              {isPending && <Loader2 className='mr-2 h-4 w-4 animate-spin' />}
              {t('kanban:column.create')}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}

export default ColumnCreateModal
