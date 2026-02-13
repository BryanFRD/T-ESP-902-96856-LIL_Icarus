import { useDroppable } from '@dnd-kit/core'
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import type { BoardColumn } from '@/domain/types/Kanban'

import KanbanCard from './KanbanCard'

type KanbanColumnProps = {
  column: BoardColumn
  onAddTicket: (columnId: string) => void
  onTicketClick: (ticketId: string) => void
  canAddTicket?: boolean
  canDragTickets?: boolean
}

const KanbanColumn = ({
  column,
  onAddTicket,
  onTicketClick,
  canAddTicket = true,
  canDragTickets = true
}: KanbanColumnProps) => {
  const { t } = useTranslation()
  const { setNodeRef, isOver } = useDroppable({
    id: column.id
  })

  return (
    <div className='flex h-full min-w-80 flex-col rounded-lg border border-gray-200 bg-gray-50 dark:border-gray-700 dark:bg-gray-800/50'>
      <div className='flex items-center justify-between border-b border-gray-200 p-4 dark:border-gray-700'>
        <div className='flex items-center gap-2'>
          <h3 className='font-semibold text-gray-900 dark:text-gray-100'>
            {column.name}
          </h3>
          <span className='flex h-6 w-6 items-center justify-center rounded-full bg-gray-200 text-xs font-medium text-gray-700 dark:bg-gray-700 dark:text-gray-300'>
            {column.tickets.length}
          </span>
        </div>
        {canAddTicket && (
          <button
            type='button'
            onClick={() => onAddTicket(column.id)}
            className='rounded-md p-1 text-gray-400 transition-colors hover:bg-gray-200 hover:text-gray-600 dark:hover:bg-gray-700 dark:hover:text-gray-300'
            title={t('kanban:addTicket')}
          >
            <Plus className='h-5 w-5' />
          </button>
        )}
      </div>

      <div
        ref={setNodeRef}
        className={twMerge(
          'flex-1 space-y-3 overflow-y-auto p-4',
          isOver && 'bg-orange-50 dark:bg-orange-900/10'
        )}
      >
        <SortableContext
          items={column.tickets.map(t => t.id)}
          strategy={verticalListSortingStrategy}
        >
          {column.tickets.map(ticket => (
            <KanbanCard
              key={ticket.id}
              ticket={ticket}
              onClick={() => onTicketClick(ticket.id)}
              canDrag={canDragTickets}
            />
          ))}
        </SortableContext>

        {column.tickets.length === 0 && (
          <div className='flex flex-col items-center justify-center py-8 text-center'>
            <p className='text-sm text-gray-400 dark:text-gray-500'>
              {t('kanban:emptyColumn')}
            </p>
          </div>
        )}
      </div>
    </div>
  )
}

export default KanbanColumn
