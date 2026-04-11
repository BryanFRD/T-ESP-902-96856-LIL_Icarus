import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { CheckSquare, Clock, MessageSquare, Paperclip } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import type { TicketSummary } from '@/domain/types/kanban.ts'

import TicketLabelBadge from './ticket-label-badge.tsx'
import TicketPriorityBadge from './ticket-priority-badge.tsx'

type KanbanCardProps = {
  ticket: TicketSummary
  onClick: () => void
  canDrag?: boolean
}

const KanbanCard = ({ ticket, onClick, canDrag = true }: KanbanCardProps) => {
  const { t } = useTranslation()
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging
  } = useSortable({
    id: ticket.id,
    disabled: !canDrag,
    transition: {
      duration: 200,
      easing: 'cubic-bezier(0.25, 1, 0.5, 1)'
    }
  })

  const style = {
    transform: CSS.Transform.toString(transform),
    transition: isDragging ? 'none' : transition
  }

  const isDueSoon = () => {
    if (!ticket.dueDate) return false
    const dueDate = new Date(ticket.dueDate)
    const now = new Date()
    const diffDays = Math.ceil(
      (dueDate.getTime() - now.getTime()) / (1000 * 60 * 60 * 24)
    )
    return diffDays <= 3 && diffDays >= 0
  }

  const isOverdue = () => {
    if (!ticket.dueDate) return false
    return new Date(ticket.dueDate) < new Date()
  }

  return (
    <div
      ref={setNodeRef}
      style={style}
      {...attributes}
      {...listeners}
      className={twMerge(
        'bg-card hover:ring-foreground/10 group relative flex flex-col gap-2.5 rounded-lg border p-3 transition-all hover:ring-1',
        isDragging && 'ring-primary z-50 rotate-2 opacity-90 shadow-xl ring-2',
        canDrag ? 'cursor-grab active:cursor-grabbing' : 'cursor-default'
      )}
      onClick={onClick}
    >
      <div className='flex items-start justify-between gap-2'>
        <span className='hover:text-primary text-sm leading-5 font-medium'>
          {ticket.title}
        </span>
        <TicketPriorityBadge priority={ticket.priority} variant='icon' />
      </div>

      {ticket.labels.length > 0 && (
        <div className='flex flex-wrap gap-1'>
          {ticket.labels.map(label => (
            <TicketLabelBadge key={label.id} label={label} size='sm' />
          ))}
        </div>
      )}

      <div className='flex items-center justify-between'>
        <div className='text-muted-foreground flex items-center gap-3'>
          {ticket.subtaskCount > 0 && (
            <div
              className='flex items-center gap-0.5'
              title={t('kanban:tooltips.subtasks')}
            >
              <CheckSquare className='h-3.5 w-3.5' />
              <span className='text-xs font-medium'>{ticket.subtaskCount}</span>
            </div>
          )}
          {ticket.commentCount > 0 && (
            <div
              className='flex items-center gap-0.5'
              title={t('kanban:tooltips.comments')}
            >
              <MessageSquare className='h-3.5 w-3.5' />
              <span className='text-xs'>{ticket.commentCount}</span>
            </div>
          )}
          {ticket.attachmentCount > 0 && (
            <div
              className='flex items-center gap-0.5'
              title={t('kanban:tooltips.attachments')}
            >
              <Paperclip className='h-3.5 w-3.5' />
              <span className='text-xs'>{ticket.attachmentCount}</span>
            </div>
          )}
        </div>

        <div className='flex items-center gap-2'>
          {ticket.dueDate && (
            <div
              className={twMerge(
                'flex items-center gap-1 rounded px-1.5 py-0.5 text-[11px] font-medium',
                isOverdue()
                  ? 'bg-red-500/10 text-red-500'
                  : isDueSoon()
                    ? 'bg-amber-500/10 text-amber-500'
                    : 'text-muted-foreground'
              )}
            >
              {(isOverdue() || isDueSoon()) && <Clock className='h-3 w-3' />}
              <span>
                {format(new Date(ticket.dueDate), 'd MMM', { locale: fr })}
              </span>
            </div>
          )}

          {ticket.assignee && (
            <Avatar
              className='h-6 w-6'
              title={`${ticket.assignee.firstName} ${ticket.assignee.lastName}`}
            >
              <AvatarFallback className='bg-primary/15 text-primary text-[10px] font-semibold'>
                {ticket.assignee.firstName[0]}
                {ticket.assignee.lastName[0]}
              </AvatarFallback>
            </Avatar>
          )}
        </div>
      </div>
    </div>
  )
}

export default KanbanCard
