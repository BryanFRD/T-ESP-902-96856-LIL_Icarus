import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import {
  Calendar,
  CheckSquare,
  Clock,
  MessageSquare,
  Paperclip,
  Trash2,
  User
} from 'lucide-react'
import { useEffect, useState } from 'react'
import { Controller, useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import DatePicker from '@/components/oldUi/DatePicker'
import Select, { type SelectOption } from '@/components/oldUi/Select'
import Textarea from '@/components/oldUi/Textarea'
import Button from '@/components/oldUi/button/Button'
import Modal from '@/components/oldUi/modal/Modal'
import ModalConfirm from '@/components/oldUi/modal/ModalConfirm.tsx'
import type { LabelResponse, TicketUser } from '@/domain/types/kanban.ts'
import { ticketPriorities } from '@/domain/validation/kanban-schema.ts'
import {
  useDeleteTicket,
  useUpdateTicket
} from '@/hooks/kanban/use-ticket-mutation.ts'
import { useTicket } from '@/hooks/kanban/use-ticket.ts'

import TicketLabelBadge from './ticket-label-badge.tsx'
import TicketPriorityBadge from './ticket-priority-badge.tsx'

type FormData = {
  title?: string | null
  description?: string | null
  assigneeId?: string | null
  priority?: 'LOW' | 'MEDIUM' | 'HIGH' | 'CRITICAL' | null
  dueDate?: Date | null
  parentId?: string | null
}

type TicketModalProps = {
  isOpen: boolean
  onClose: () => void
  auditId: string
  ticketId: string
  availableUsers?: TicketUser[]
  availableLabels?: LabelResponse[]
}

const TicketModal = ({
  isOpen,
  onClose,
  auditId,
  ticketId,
  availableUsers = []
}: TicketModalProps) => {
  const { t } = useTranslation()
  const { data: ticket, isLoading } = useTicket(auditId, ticketId)
  const { mutateAsync: updateTicket, isPending: isUpdating } = useUpdateTicket(
    auditId,
    ticketId
  )
  const { mutateAsync: deleteTicket, isPending: isDeleting } =
    useDeleteTicket(auditId)

  const [isEditing, setIsEditing] = useState(false)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false)

  const {
    register,
    handleSubmit,
    control,
    reset,
    formState: { errors }
  } = useForm<FormData>()

  useEffect(() => {
    if (ticket) {
      reset({
        title: ticket.title,
        description: ticket.description,
        assigneeId: ticket.assignee?.id || null,
        priority: ticket.priority,
        dueDate: ticket.dueDate ? new Date(ticket.dueDate) : null,
        parentId: ticket.parentId
      })
    }
  }, [ticket, reset])

  const priorityOptions: SelectOption[] = ticketPriorities.map(priority => ({
    value: priority,
    label: t(`kanban:priority.${priority.toLowerCase()}`)
  }))

  const userOptions: SelectOption[] = availableUsers.map(user => ({
    value: user.id,
    label: `${user.firstName} ${user.lastName}`
  }))

  const onSubmit = async (data: FormData) => {
    try {
      const payload = {
        title: data.title,
        description: data.description,
        assigneeId: data.assigneeId,
        priority: data.priority,
        dueDate: data.dueDate ? data.dueDate.toISOString() : null,
        parentId: data.parentId
      }
      await updateTicket(payload)
      setIsEditing(false)
      toast.success(t('kanban:ticket.updated'))
    } catch {
      // Error handled by mutation
    }
  }

  const handleDelete = async () => {
    try {
      await deleteTicket(ticketId)
      setShowDeleteConfirm(false)
      onClose()
    } catch {
      // Error handled by mutation
    }
  }

  if (isLoading || !ticket) {
    return (
      <Modal isOpen={isOpen} onClose={onClose} title={t('kanban:loading')}>
        <div className='flex items-center justify-center py-8'>
          <div className='h-8 w-8 animate-spin rounded-full border-4 border-gray-200 border-t-orange-500' />
        </div>
      </Modal>
    )
  }

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      title={isEditing ? t('kanban:editTicket') : ticket.title}
      size='xl'
    >
      <div className='space-y-6'>
        {!isEditing && (
          <div className='space-y-4'>
            <div className='flex items-start justify-between'>
              <div className='flex-1'>
                <h2 className='text-xl font-semibold text-gray-900 dark:text-gray-100'>
                  {ticket.title}
                </h2>
                {ticket.description && (
                  <p className='mt-2 text-sm whitespace-pre-wrap text-gray-600 dark:text-gray-400'>
                    {ticket.description}
                  </p>
                )}
              </div>
              <Button
                variant='ghost'
                size='sm'
                onClick={() => setIsEditing(true)}
              >
                {t('common:edit')}
              </Button>
            </div>

            <div className='grid grid-cols-2 gap-4 rounded-lg bg-gray-50 p-4 dark:bg-gray-800/50'>
              <div className='space-y-3'>
                <div className='flex items-center gap-2 text-sm'>
                  <User className='h-4 w-4 text-gray-400' />
                  <span className='text-gray-600 dark:text-gray-400'>
                    {t('kanban:ticket.assignee')}:
                  </span>
                  {ticket.assignee ? (
                    <div className='flex items-center gap-2'>
                      {ticket.assignee.avatarUrl ? (
                        <img
                          src={ticket.assignee.avatarUrl}
                          alt={`${ticket.assignee.firstName} ${ticket.assignee.lastName}`}
                          className='h-5 w-5 rounded-full'
                        />
                      ) : (
                        <div className='flex h-5 w-5 items-center justify-center rounded-full bg-orange-100 text-xs font-semibold text-orange-600 dark:bg-orange-900/20 dark:text-orange-400'>
                          {ticket.assignee.firstName[0]}
                          {ticket.assignee.lastName[0]}
                        </div>
                      )}
                      <span className='text-gray-900 dark:text-gray-100'>
                        {ticket.assignee.firstName} {ticket.assignee.lastName}
                      </span>
                    </div>
                  ) : (
                    <span className='text-gray-400'>
                      {t('kanban:ticket.unassigned')}
                    </span>
                  )}
                </div>

                <div className='flex items-center gap-2 text-sm'>
                  <Calendar className='h-4 w-4 text-gray-400' />
                  <span className='text-gray-600 dark:text-gray-400'>
                    {t('kanban:ticket.dueDate')}:
                  </span>
                  {ticket.dueDate ? (
                    <span className='text-gray-900 dark:text-gray-100'>
                      {format(new Date(ticket.dueDate), 'dd MMMM yyyy', {
                        locale: fr
                      })}
                    </span>
                  ) : (
                    <span className='text-gray-400'>
                      {t('kanban:ticket.noDueDate')}
                    </span>
                  )}
                </div>
              </div>

              <div className='space-y-3'>
                <div className='flex items-center gap-2 text-sm'>
                  <Clock className='h-4 w-4 text-gray-400' />
                  <span className='text-gray-600 dark:text-gray-400'>
                    {t('kanban:ticket.priority')}:
                  </span>
                  <TicketPriorityBadge priority={ticket.priority} />
                </div>

                <div className='flex items-center gap-2 text-sm'>
                  <MessageSquare className='h-4 w-4 text-gray-400' />
                  <span className='text-gray-600 dark:text-gray-400'>
                    {t('kanban:comments')}:
                  </span>
                  <span className='text-gray-900 dark:text-gray-100'>
                    {ticket.commentCount}
                  </span>
                </div>
              </div>
            </div>

            {ticket.labels.length > 0 && (
              <div>
                <h3 className='mb-2 text-sm font-medium text-gray-700 dark:text-gray-300'>
                  {t('kanban:labels')}
                </h3>
                <div className='flex flex-wrap gap-2'>
                  {ticket.labels.map(label => (
                    <TicketLabelBadge key={label.id} label={label} />
                  ))}
                </div>
              </div>
            )}

            <div className='flex items-center gap-6 text-sm text-gray-500'>
              {ticket.subtaskCount > 0 && (
                <div className='flex items-center gap-2'>
                  <CheckSquare className='h-4 w-4' />
                  <span>
                    {ticket.subtaskCount} {t('kanban:subtasks')}
                  </span>
                </div>
              )}
              {ticket.attachmentCount > 0 && (
                <div className='flex items-center gap-2'>
                  <Paperclip className='h-4 w-4' />
                  <span>
                    {ticket.attachmentCount} {t('kanban:attachments')}
                  </span>
                </div>
              )}
            </div>
          </div>
        )}

        {isEditing && (
          <form onSubmit={handleSubmit(onSubmit)} className='space-y-4'>
            <div className='space-y-1.5'>
              <label className='text-sm font-medium text-gray-700 dark:text-gray-200'>
                {t('kanban:ticket.title')}
              </label>
              <input
                type='text'
                {...register('title')}
                placeholder={t('kanban:ticket.titlePlaceholder')}
                className='w-full rounded-md border-2 border-gray-200 bg-white px-3 py-2 text-sm text-gray-900 transition-all duration-200 outline-none placeholder:text-gray-400 focus:border-orange-500 dark:border-gray-700 dark:bg-gray-900 dark:text-gray-100 dark:placeholder:text-gray-500 dark:focus:border-orange-500'
              />
              {errors.title?.message && (
                <p className='text-sm text-red-600 dark:text-red-400'>
                  {errors.title.message}
                </p>
              )}
            </div>

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
                        option &&
                          typeof option === 'object' &&
                          'value' in option
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
                  value={field.value || null}
                  onChange={field.onChange}
                  error={errors.dueDate?.message}
                />
              )}
            />

            <div className='flex justify-between gap-3 pt-4'>
              <Button
                type='button'
                variant='danger'
                onClick={() => setShowDeleteConfirm(true)}
                disabled={isDeleting || isUpdating}
              >
                <Trash2 className='h-4 w-4' />
                {t('common:delete')}
              </Button>
              <div className='flex gap-3'>
                <Button
                  type='button'
                  variant='ghost'
                  onClick={() => setIsEditing(false)}
                  disabled={isUpdating}
                >
                  {t('common:cancel')}
                </Button>
                <Button type='submit' variant='primary' isLoading={isUpdating}>
                  {t('common:save')}
                </Button>
              </div>
            </div>
          </form>
        )}

        {!isEditing && (
          <div className='border-t border-gray-200 pt-4 text-xs text-gray-500 dark:border-gray-700'>
            <div className='flex justify-between'>
              <span>
                {t('kanban:ticket.createdBy')} {ticket.createdBy.firstName}{' '}
                {ticket.createdBy.lastName}
              </span>
              <span>
                {format(new Date(ticket.createdAt), 'dd/MM/yyyy HH:mm', {
                  locale: fr
                })}
              </span>
            </div>
          </div>
        )}
      </div>

      <ModalConfirm
        isOpen={showDeleteConfirm}
        onClose={() => setShowDeleteConfirm(false)}
        onConfirm={handleDelete}
        title={t('kanban:ticket.deleteTitle')}
        description={t('kanban:ticket.deleteDescription')}
        confirmText={t('common:delete')}
        confirmMatch={ticket.title}
        isLoading={isDeleting}
        variant='danger'
      />
    </Modal>
  )
}

export default TicketModal
