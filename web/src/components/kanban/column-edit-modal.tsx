import { zodResolver } from '@hookform/resolvers/zod'
import { Loader2 } from 'lucide-react'
import { useEffect } from 'react'
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
    defaultValues: { name: '' }
  })

  useEffect(() => {
    if (column) {
      reset({ name: column.name })
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
    <Dialog open={isOpen} onOpenChange={open => !open && handleClose()}>
      <DialogContent className='sm:max-w-md'>
        <DialogHeader>
          <DialogTitle>{t('kanban:column.edit')}</DialogTitle>
        </DialogHeader>

        <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
          <div className='space-y-2'>
            <Label htmlFor='col-edit-name'>{t('kanban:column.name')}</Label>
            <Input
              id='col-edit-name'
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
              {t('common:actions.save')}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  )
}

export default ColumnEditModal
