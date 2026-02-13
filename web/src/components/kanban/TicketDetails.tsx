import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { Calendar, Check, Edit2, Flag, Layers, User, X } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import DatePicker from '@/components/ui/DatePicker'
import Select, { type SelectOption } from '@/components/ui/Select'
import Textarea from '@/components/ui/Textarea'
import Button from '@/components/ui/button/Button'
import { useUserContext } from '@/context/UserContext'
import { canEditTicket } from '@/domain/permissions/kanbanPermissions'
import type {
  LabelResponse,
  TicketResponse,
  TicketUser
} from '@/domain/types/Kanban'
import { ticketPriorities } from '@/domain/validation/kanbanSchema'
import { useAuditRole } from '@/hooks/kanban/useAuditRole'
import {
  useAddLabelToTicket,
  useRemoveLabelFromTicket
} from '@/hooks/kanban/useLabelMutations'
import { useUpdateTicket } from '@/hooks/kanban/useTicketMutations'

import TicketLabelBadge from './TicketLabelBadge'
import TicketPriorityBadge from './TicketPriorityBadge'

type TicketDetailsProps = {
  ticket: TicketResponse
  auditId: string
  teamId: string
  availableUsers: TicketUser[]
  availableLabels: LabelResponse[]
}

const TicketDetails = ({
  ticket,
  auditId,
  teamId,
  availableUsers,
  availableLabels
}: TicketDetailsProps) => {
  const { t } = useTranslation()
  const { user } = useUserContext()
  const { role } = useAuditRole(teamId)
  const { mutateAsync: updateTicket } = useUpdateTicket(auditId, ticket.id)
  const { mutateAsync: addLabel } = useAddLabelToTicket(auditId, ticket.id)
  const { mutateAsync: removeLabel } = useRemoveLabelFromTicket(
    auditId,
    ticket.id
  )

  const [isEditingTitle, setIsEditingTitle] = useState(false)
  const [isEditingDescription, setIsEditingDescription] = useState(false)
  const [titleValue, setTitleValue] = useState(ticket.title)
  const [descriptionValue, setDescriptionValue] = useState(
    ticket.description || ''
  )

  const canEdit = role && user ? canEditTicket(role) : false

  const priorityOptions: SelectOption[] = ticketPriorities.map(priority => ({
    value: priority,
    label: t(`kanban:priority.${priority.toLowerCase()}`)
  }))

  const userOptions: SelectOption[] = availableUsers.map(user => ({
    value: user.id,
    label: `${user.firstName} ${user.lastName}`
  }))

  const handleUpdateField = async (field: string, value: unknown) => {
    try {
      await updateTicket({ [field]: value })
    } catch {
      // Error handled by mutation
    }
  }

  const handleSaveTitle = async () => {
    if (titleValue.trim() && titleValue !== ticket.title) {
      await handleUpdateField('title', titleValue.trim())
    }
    setIsEditingTitle(false)
  }

  const handleSaveDescription = async () => {
    if (descriptionValue !== ticket.description) {
      await handleUpdateField('description', descriptionValue || null)
    }
    setIsEditingDescription(false)
  }

  const handleAddLabel = async (labelId: string) => {
    if (!ticket.labels.some(l => l.id === labelId)) {
      await addLabel(labelId)
    }
  }

  const handleRemoveLabel = async (labelId: string) => {
    await removeLabel(labelId)
  }

  return (
    <div className='space-y-6'>
      <div>
        {isEditingTitle ? (
          <div
            className='flex items-center gap-2'
            onClick={e => e.stopPropagation()}
          >
            <input
              type='text'
              value={titleValue}
              onChange={e => setTitleValue(e.target.value)}
              onClick={e => e.stopPropagation()}
              className='flex-1 rounded border border-gray-300 px-3 py-2 text-xl font-semibold text-gray-900 outline-none focus:border-orange-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-100'
              autoFocus
              onKeyDown={e => {
                if (e.key === 'Enter') handleSaveTitle()
                if (e.key === 'Escape') {
                  setTitleValue(ticket.title)
                  setIsEditingTitle(false)
                }
              }}
            />
            <Button variant='ghost' size='sm' onClick={handleSaveTitle}>
              <Check className='h-4 w-4' />
            </Button>
            <Button
              variant='ghost'
              size='sm'
              onClick={() => {
                setTitleValue(ticket.title)
                setIsEditingTitle(false)
              }}
            >
              <X className='h-4 w-4' />
            </Button>
          </div>
        ) : (
          <div className='group flex items-start gap-2'>
            <h1 className='flex-1 text-2xl font-semibold text-gray-900 dark:text-gray-100'>
              {ticket.title}
            </h1>
            {canEdit && (
              <button
                onClick={() => setIsEditingTitle(true)}
                className='opacity-0 transition-opacity group-hover:opacity-100'
              >
                <Edit2 className='h-4 w-4 text-gray-400 hover:text-gray-600' />
              </button>
            )}
          </div>
        )}
      </div>

      <div className='grid grid-cols-2 gap-4'>
        <div className='space-y-1'>
          <label className='flex items-center gap-2 text-sm font-medium text-gray-600 dark:text-gray-400'>
            <User className='h-4 w-4' />
            {t('kanban:ticket.assignee')}
          </label>
          {canEdit ? (
            <Select
              options={[
                { value: '', label: t('kanban:ticket.unassigned') },
                ...userOptions
              ]}
              value={
                ticket.assignee
                  ? userOptions.find(o => o.value === ticket.assignee?.id)
                  : { value: '', label: t('kanban:ticket.unassigned') }
              }
              onChange={option => {
                const value =
                  option && typeof option === 'object' && 'value' in option
                    ? option.value
                    : null
                handleUpdateField('assigneeId', value || null)
              }}
            />
          ) : (
            <div className='text-sm text-gray-900 dark:text-gray-100'>
              {ticket.assignee
                ? `${ticket.assignee.firstName} ${ticket.assignee.lastName}`
                : t('kanban:ticket.unassigned')}
            </div>
          )}
        </div>

        <div className='space-y-1'>
          <label className='flex items-center gap-2 text-sm font-medium text-gray-600 dark:text-gray-400'>
            <Flag className='h-4 w-4' />
            {t('kanban:ticket.priority')}
          </label>
          {canEdit ? (
            <Select
              options={priorityOptions}
              value={priorityOptions.find(o => o.value === ticket.priority)}
              onChange={option => {
                const value =
                  option && typeof option === 'object' && 'value' in option
                    ? option.value
                    : null
                handleUpdateField('priority', value)
              }}
            />
          ) : (
            <div>
              <TicketPriorityBadge priority={ticket.priority} />
            </div>
          )}
        </div>

        <div className='space-y-1'>
          <label className='flex items-center gap-2 text-sm font-medium text-gray-600 dark:text-gray-400'>
            <Calendar className='h-4 w-4' />
            {t('kanban:ticket.dueDate')}
          </label>
          {canEdit ? (
            <DatePicker
              value={ticket.dueDate ? new Date(ticket.dueDate) : null}
              onChange={date =>
                handleUpdateField('dueDate', date ? date.toISOString() : null)
              }
            />
          ) : (
            <div className='text-sm text-gray-900 dark:text-gray-100'>
              {ticket.dueDate
                ? format(new Date(ticket.dueDate), 'dd MMMM yyyy', {
                    locale: fr
                  })
                : t('kanban:ticket.noDueDate')}
            </div>
          )}
        </div>
      </div>

      <div className='space-y-2'>
        <label className='flex items-center gap-2 text-sm font-medium text-gray-600 dark:text-gray-400'>
          <Layers className='h-4 w-4' />
          {t('kanban:labels')}
        </label>
        <div className='flex flex-wrap gap-2'>
          {ticket.labels.map(label => (
            <TicketLabelBadge
              key={label.id}
              label={label}
              onRemove={canEdit ? () => handleRemoveLabel(label.id) : undefined}
            />
          ))}
          {canEdit &&
            availableLabels.some(
              l => !ticket.labels.some(tl => tl.id === l.id)
            ) && (
              <Select
                options={availableLabels
                  .filter(l => !ticket.labels.some(tl => tl.id === l.id))
                  .map(l => ({ value: l.id, label: l.name }))}
                onChange={option => {
                  if (
                    option &&
                    typeof option === 'object' &&
                    'value' in option
                  ) {
                    handleAddLabel(option.value as string)
                  }
                }}
                placeholder={t('kanban:label.add')}
              />
            )}
        </div>
      </div>

      <div className='space-y-2'>
        <label className='text-sm font-medium text-gray-600 dark:text-gray-400'>
          {t('kanban:ticket.description')}
        </label>
        {isEditingDescription ? (
          <div className='space-y-2' onClick={e => e.stopPropagation()}>
            <Textarea
              value={descriptionValue}
              onChange={e => setDescriptionValue(e.target.value)}
              onClick={e => e.stopPropagation()}
              rows={6}
              placeholder={t('kanban:ticket.descriptionPlaceholder')}
            />
            <div className='flex gap-2'>
              <Button size='sm' onClick={handleSaveDescription}>
                {t('common:save')}
              </Button>
              <Button
                variant='ghost'
                size='sm'
                onClick={() => {
                  setDescriptionValue(ticket.description || '')
                  setIsEditingDescription(false)
                }}
              >
                {t('common:cancel')}
              </Button>
            </div>
          </div>
        ) : (
          <div className='group relative'>
            <div className='min-h-25 rounded border border-gray-200 bg-gray-50 px-4 py-3 text-sm text-gray-700 dark:border-gray-700 dark:bg-gray-800/50 dark:text-gray-300'>
              {ticket.description || (
                <span className='text-gray-400'>
                  {t('kanban:ticket.noDescription')}
                </span>
              )}
            </div>
            {canEdit && (
              <button
                onClick={() => setIsEditingDescription(true)}
                className='absolute top-2 right-2 opacity-0 transition-opacity group-hover:opacity-100'
              >
                <Edit2 className='h-4 w-4 text-gray-400 hover:text-gray-600' />
              </button>
            )}
          </div>
        )}
      </div>

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
    </div>
  )
}

export default TicketDetails
