import { useDroppable } from '@dnd-kit/core'
import { SortableContext, verticalListSortingStrategy } from '@dnd-kit/sortable'
import { Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import type { BoardColumn } from '@/domain/types/kanban.ts'

import KanbanCard from './kanban-card.tsx'

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
    <div className='bg-muted/30 flex h-full min-w-80 flex-col rounded-xl'>
      <div className='flex items-center justify-between px-3 py-3'>
        <div className='flex items-center gap-2'>
          <h3 className='text-muted-foreground text-sm font-semibold tracking-wide uppercase'>
            {column.name}
          </h3>
          <Badge
            variant='outline'
            className='text-muted-foreground text-[11px]'
          >
            {column.tickets.length}
          </Badge>
        </div>
        {canAddTicket && (
          <Button
            variant='ghost'
            size='icon-sm'
            onClick={() => onAddTicket(column.id)}
            title={t('kanban:addTicket')}
          >
            <Plus className='h-4 w-4' />
          </Button>
        )}
      </div>

      <div
        ref={setNodeRef}
        className={twMerge(
          'flex-1 space-y-2 overflow-y-auto px-2 pb-2',
          isOver && 'bg-primary/5'
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
          <div className='text-muted-foreground flex flex-col items-center justify-center py-8 text-center'>
            <p className='text-sm'>{t('kanban:emptyColumn')}</p>
          </div>
        )}
      </div>
    </div>
  )
}

export default KanbanColumn
