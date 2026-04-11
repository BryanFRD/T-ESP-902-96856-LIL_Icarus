import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import {
  Calendar,
  Check,
  ChevronDown,
  ChevronUp,
  ChevronsUp,
  Edit2,
  Equal,
  Layers,
  User,
  X
} from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import { Textarea } from '@/components/ui/textarea'
import { useUserContext } from '@/context/user-context.tsx'
import { canEditTicket } from '@/domain/permissions/kanban-permissions.ts'
import type {
  LabelResponse,
  TicketResponse,
  TicketUser
} from '@/domain/types/kanban.ts'
import { useAuditRole } from '@/hooks/kanban/use-audit-role.ts'
import {
  useAddLabelToTicket,
  useRemoveLabelFromTicket
} from '@/hooks/kanban/use-label-mutation.ts'
import { useUpdateTicket } from '@/hooks/kanban/use-ticket-mutation.ts'

import TicketLabelBadge from './ticket-label-badge.tsx'
import TicketPriorityBadge from './ticket-priority-badge.tsx'

const PRIORITY_CONFIG = {
  CRITICAL: { icon: ChevronsUp, color: 'text-red-500' },
  HIGH: { icon: ChevronUp, color: 'text-red-500' },
  MEDIUM: { icon: Equal, color: 'text-orange-500' },
  LOW: { icon: ChevronDown, color: 'text-blue-500' }
} as const

const PRIORITIES = ['CRITICAL', 'HIGH', 'MEDIUM', 'LOW'] as const

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
      {/* Title */}
      <div>
        {isEditingTitle ? (
          <div className='flex items-center gap-2'>
            <Input
              value={titleValue}
              onChange={e => setTitleValue(e.target.value)}
              className='text-xl font-semibold'
              autoFocus
              onKeyDown={e => {
                if (e.key === 'Enter') handleSaveTitle()
                if (e.key === 'Escape') {
                  setTitleValue(ticket.title)
                  setIsEditingTitle(false)
                }
              }}
            />
            <Button variant='ghost' size='icon-sm' onClick={handleSaveTitle}>
              <Check className='h-4 w-4' />
            </Button>
            <Button
              variant='ghost'
              size='icon-sm'
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
            <h1 className='flex-1 text-xl font-semibold'>{ticket.title}</h1>
            {canEdit && (
              <button
                onClick={() => setIsEditingTitle(true)}
                className='opacity-0 transition-opacity group-hover:opacity-100'
              >
                <Edit2 className='text-muted-foreground hover:text-foreground h-4 w-4' />
              </button>
            )}
          </div>
        )}
      </div>

      {/* Fields grid */}
      <div className='grid grid-cols-2 gap-4'>
        {/* Assignee */}
        <div className='space-y-2'>
          <Label className='text-muted-foreground flex items-center gap-2'>
            <User className='h-4 w-4' />
            {t('kanban:ticket.assignee')}
          </Label>
          {canEdit ? (
            <Select
              value={ticket.assignee?.id ?? 'none'}
              onValueChange={v =>
                handleUpdateField('assigneeId', v === 'none' ? null : v)
              }
            >
              <SelectTrigger>
                <SelectValue>
                  {ticket.assignee ? (
                    <span className='flex items-center gap-2'>
                      <Avatar className='h-5 w-5'>
                        <AvatarFallback className='bg-primary/10 text-primary text-[9px]'>
                          {ticket.assignee.firstName[0]}
                          {ticket.assignee.lastName[0]}
                        </AvatarFallback>
                      </Avatar>
                      {ticket.assignee.firstName} {ticket.assignee.lastName}
                    </span>
                  ) : (
                    t('kanban:ticket.unassigned')
                  )}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value='none'>
                  <span className='text-muted-foreground'>
                    {t('kanban:ticket.unassigned')}
                  </span>
                </SelectItem>
                {availableUsers.map(u => (
                  <SelectItem key={u.id} value={u.id}>
                    <span className='flex items-center gap-2'>
                      <Avatar className='h-5 w-5'>
                        <AvatarFallback className='bg-primary/10 text-primary text-[9px]'>
                          {u.firstName[0]}
                          {u.lastName[0]}
                        </AvatarFallback>
                      </Avatar>
                      {u.firstName} {u.lastName}
                    </span>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          ) : (
            <p className='text-sm'>
              {ticket.assignee
                ? `${ticket.assignee.firstName} ${ticket.assignee.lastName}`
                : t('kanban:ticket.unassigned')}
            </p>
          )}
        </div>

        {/* Priority */}
        <div className='space-y-2'>
          <Label className='text-muted-foreground flex items-center gap-2'>
            <ChevronsUp className='h-4 w-4' />
            {t('kanban:ticket.priority')}
          </Label>
          {canEdit ? (
            <Select
              value={ticket.priority}
              onValueChange={v => handleUpdateField('priority', v)}
            >
              <SelectTrigger>
                <SelectValue>
                  {(() => {
                    const config =
                      PRIORITY_CONFIG[
                        ticket.priority as keyof typeof PRIORITY_CONFIG
                      ]
                    if (!config) return ticket.priority
                    const Icon = config.icon
                    return (
                      <span
                        className={`flex items-center gap-2 ${config.color}`}
                      >
                        <Icon className='h-4 w-4' strokeWidth={2.5} />
                        {t(`kanban:priority.${ticket.priority.toLowerCase()}`)}
                      </span>
                    )
                  })()}
                </SelectValue>
              </SelectTrigger>
              <SelectContent>
                {PRIORITIES.map(p => {
                  const config = PRIORITY_CONFIG[p]
                  const Icon = config.icon
                  return (
                    <SelectItem key={p} value={p}>
                      <span
                        className={`flex items-center gap-2 ${config.color}`}
                      >
                        <Icon className='h-4 w-4' strokeWidth={2.5} />
                        {t(`kanban:priority.${p.toLowerCase()}`)}
                      </span>
                    </SelectItem>
                  )
                })}
              </SelectContent>
            </Select>
          ) : (
            <TicketPriorityBadge priority={ticket.priority} />
          )}
        </div>

        {/* Due Date */}
        <div className='space-y-2'>
          <Label className='text-muted-foreground flex items-center gap-2'>
            <Calendar className='h-4 w-4' />
            {t('kanban:ticket.dueDate')}
          </Label>
          {canEdit ? (
            <Input
              type='date'
              value={
                ticket.dueDate
                  ? new Date(ticket.dueDate).toISOString().split('T')[0]
                  : ''
              }
              onChange={e =>
                handleUpdateField(
                  'dueDate',
                  e.target.value ? new Date(e.target.value).toISOString() : null
                )
              }
            />
          ) : (
            <p className='text-sm'>
              {ticket.dueDate
                ? format(new Date(ticket.dueDate), 'dd MMMM yyyy', {
                    locale: fr
                  })
                : t('kanban:ticket.noDueDate')}
            </p>
          )}
        </div>
      </div>

      {/* Labels */}
      <div className='space-y-2'>
        <Label className='text-muted-foreground flex items-center gap-2'>
          <Layers className='h-4 w-4' />
          {t('kanban:labels')}
        </Label>
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
              <Select onValueChange={v => handleAddLabel(v)}>
                <SelectTrigger className='h-7 w-auto min-w-[120px]'>
                  <SelectValue placeholder={t('kanban:label.add')} />
                </SelectTrigger>
                <SelectContent>
                  {availableLabels
                    .filter(l => !ticket.labels.some(tl => tl.id === l.id))
                    .map(l => (
                      <SelectItem key={l.id} value={l.id}>
                        {l.name}
                      </SelectItem>
                    ))}
                </SelectContent>
              </Select>
            )}
        </div>
      </div>

      {/* Description */}
      <div className='space-y-2'>
        <Label className='text-muted-foreground'>
          {t('kanban:ticket.description')}
        </Label>
        {isEditingDescription ? (
          <div className='space-y-2'>
            <Textarea
              value={descriptionValue}
              onChange={e => setDescriptionValue(e.target.value)}
              rows={6}
              placeholder={t('kanban:ticket.descriptionPlaceholder')}
            />
            <div className='flex gap-2'>
              <Button size='sm' onClick={handleSaveDescription}>
                {t('common:actions.save')}
              </Button>
              <Button
                variant='outline'
                size='sm'
                onClick={() => {
                  setDescriptionValue(ticket.description || '')
                  setIsEditingDescription(false)
                }}
              >
                {t('common:actions.cancel')}
              </Button>
            </div>
          </div>
        ) : (
          <div className='group relative'>
            <div className='bg-muted/50 text-foreground min-h-20 rounded-lg border px-4 py-3 text-sm'>
              {ticket.description || (
                <span className='text-muted-foreground'>
                  {t('kanban:ticket.noDescription')}
                </span>
              )}
            </div>
            {canEdit && (
              <button
                onClick={() => setIsEditingDescription(true)}
                className='absolute top-2 right-2 opacity-0 transition-opacity group-hover:opacity-100'
              >
                <Edit2 className='text-muted-foreground hover:text-foreground h-4 w-4' />
              </button>
            )}
          </div>
        )}
      </div>

      {/* Footer */}
      <div className='text-muted-foreground border-t pt-4 text-xs'>
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
