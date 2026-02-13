import { FileText } from 'lucide-react'
import { Controller, useForm } from 'react-hook-form'
import { useTranslation } from 'react-i18next'

import DatePicker from '@/components/ui/DatePicker'
import InputField from '@/components/ui/InputField'
import Select, { type SelectOption } from '@/components/ui/Select'
import Textarea from '@/components/ui/Textarea'
import Button from '@/components/ui/button/Button'
import Modal from '@/components/ui/modal/Modal'
import type { LabelResponse, TicketUser } from '@/domain/types/Kanban'
import { ticketPriorities } from '@/domain/validation/kanbanSchema'
import { useCreateTicket } from '@/hooks/kanban/useTicketMutations'

type TicketFormData = {
  title: string
  columnId: string
  description?: string | null
  assigneeId?: string | null
  priority?: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL' | null
  dueDate?: Date | null
  parentId?: string | null
}

type TicketCreateModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
  columnId: string
  availableUsers?: TicketUser[]
  availableLabels?: LabelResponse[]
}

const TicketCreateModal = ({
  isOpen,
  onClose,
  auditId,
  columnId,
  availableUsers = []
}: TicketCreateModalProps) => {
  const { t } = useTranslation()
  const { mutateAsync: createTicket, isPending } = useCreateTicket(auditId)

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors }
  } = useForm<TicketFormData>({
    defaultValues: {
      columnId,
      title: '',
      description: null,
      assigneeId: null,
      priority: null,
      dueDate: null,
      parentId: null
    }
  })

  const priorityOptions: SelectOption[] = ticketPriorities.map(priority => ({
    value: priority,
    label: t(`kanban:priority.${priority.toLowerCase()}`)
  }))

  const userOptions: SelectOption[] = availableUsers.map(user => ({
    value: user.id,
    label: `${user.firstName} ${user.lastName}`
  }))

  const onSubmit = async (data: TicketFormData) => {
    try {
      await createTicket({
        ...data,
        dueDate: data.dueDate ? data.dueDate.toISOString() : null
      })
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
      title={t('kanban:createTicket')}
      size='lg'
    >
      <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
        <InputField
          type='text'
          icon={FileText}
          name='title'
          register={register}
          label={t('kanban:ticket.title')}
          placeholder={t('kanban:ticket.titlePlaceholder')}
          error={errors.title?.message}
        />

        <Textarea
          label={t('kanban:ticket.description')}
          placeholder={t('kanban:ticket.descriptionPlaceholder')}
          error={errors.description?.message}
          rows={4}
          {...register('description')}
        />

        <Controller
          name='priority'
          control={control}
          render={({ field }) => (
            <Select
              label={t('kanban:ticket.priority')}
              placeholder={t('kanban:ticket.priorityPlaceholder')}
              options={priorityOptions}
              value={
                field.value
                  ? priorityOptions.find(o => o.value === field.value)
                  : undefined
              }
              onChange={option =>
                field.onChange(
                  option && typeof option === 'object' && 'value' in option
                    ? option.value
                    : null
                )
              }
              error={errors.priority?.message}
            />
          )}
        />

        {userOptions.length > 0 && (
          <Controller
            name='assigneeId'
            control={control}
            render={({ field }) => (
              <Select
                label={t('kanban:ticket.assignee')}
                placeholder={t('kanban:ticket.assigneePlaceholder')}
                options={userOptions}
                value={
                  field.value
                    ? userOptions.find(o => o.value === field.value)
                    : undefined
                }
                onChange={option =>
                  field.onChange(
                    option && typeof option === 'object' && 'value' in option
                      ? option.value
                      : null
                  )
                }
                error={errors.assigneeId?.message}
              />
            )}
          />
        )}

        <Controller
          name='dueDate'
          control={control}
          render={({ field }) => (
            <DatePicker
              label={t('kanban:ticket.dueDate')}
              placeholder={t('kanban:ticket.dueDatePlaceholder')}
              value={field.value ? new Date(field.value) : null}
              onChange={field.onChange}
              error={errors.dueDate?.message}
            />
          )}
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
            {t('kanban:createTicket')}
          </Button>
        </div>
      </form>
    </Modal>
  )
}

export default TicketCreateModal
