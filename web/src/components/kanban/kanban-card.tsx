import { useSortable } from '@dnd-kit/sortable'
import { CSS } from '@dnd-kit/utilities'
import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { CheckSquare, Clock, MessageSquare, Paperclip } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

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
        'group dark:hover:bg-gray-750 relative flex flex-col gap-2 rounded-[3px] border border-gray-200 bg-white p-3 shadow-[0px_1px_1px_#091e4240] transition-shadow hover:bg-gray-50 dark:border-gray-700 dark:bg-gray-800 dark:shadow-none',
        isDragging &&
          'z-50 rotate-2 opacity-90 shadow-xl ring-2 ring-[#0052CC]',
        canDrag ? 'cursor-grab active:cursor-grabbing' : 'cursor-default'
      )}
      onClick={onClick}
    >
      <div className='flex items-start justify-between gap-2'>
        <span className='text-[14px] leading-5 font-medium text-[#172B4D] hover:text-[#0052CC] hover:underline dark:text-gray-200'>
          {ticket.title}
        </span>
      </div>

      {ticket.labels.length > 0 && (
        <div className='flex flex-wrap gap-1'>
          {ticket.labels.map(label => (
            <TicketLabelBadge key={label.id} label={label} size='sm' />
          ))}
        </div>
      )}

      <div className='mt-1 flex items-center justify-between'>
        <div className='flex items-center gap-3'>
          <TicketPriorityBadge priority={ticket.priority} variant='icon' />

          {(ticket.subtaskCount > 0 ||
            ticket.commentCount > 0 ||
            ticket.attachmentCount > 0) && (
            <div className='flex items-center gap-3 text-gray-500 dark:text-gray-400'>
              {ticket.subtaskCount > 0 && (
                <div
                  className='flex items-center gap-0.5'
                  title={t('kanban:tooltips.subtasks')}
                >
                  <CheckSquare className='h-3.5 w-3.5' />
                  <span className='text-xs font-medium'>
                    {ticket.subtaskCount}
                  </span>
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
          )}
        </div>

        <div className='flex items-center gap-2'>
          {ticket.dueDate && (
            <div
              className={twMerge(
                'flex items-center gap-1 rounded-[3px] px-1 py-0.5 text-[11px] font-semibold',
                isOverdue()
                  ? 'bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-300'
                  : isDueSoon()
                    ? 'bg-orange-100 text-orange-700 dark:bg-orange-900/40 dark:text-orange-300'
                    : 'bg-transparent text-gray-500 dark:text-gray-400'
              )}
            >
              {(isOverdue() || isDueSoon()) && <Clock className='h-3 w-3' />}
              <span>
                {format(new Date(ticket.dueDate), 'd MMM', { locale: fr })}
              </span>
            </div>
          )}

          {ticket.assignee && (
            <div
              className='flex h-6 w-6 items-center justify-center rounded-full bg-[#0052CC] text-[10px] font-bold text-white shadow-sm ring-1 ring-white dark:ring-gray-700'
              title={`${ticket.assignee.firstName} ${ticket.assignee.lastName}`}
            >
              {ticket.assignee.firstName[0]}
              {ticket.assignee.lastName[0]}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default KanbanCard
