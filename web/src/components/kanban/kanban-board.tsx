import {
  DndContext,
  type DragEndEvent,
  DragOverlay,
  type DragStartEvent,
  PointerSensor,
  closestCorners,
  useSensor,
  useSensors
} from '@dnd-kit/core'
import { Loader2 } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { getKanbanPermissions } from '@/domain/permissions/kanban-permissions.ts'
import type { TicketSummary, TicketUser } from '@/domain/types/kanban.ts'
import useToggle from '@/hooks/common/use-toggle.ts'
import { useAuditRole } from '@/hooks/kanban/use-audit-role.ts'
import { useKanbanBoard } from '@/hooks/kanban/use-kanban-board.ts'
import { useMoveTicket } from '@/hooks/kanban/use-ticket-mutation.ts'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'

import KanbanCard from './kanban-card.tsx'
import KanbanColumn from './kanban-column.tsx'
import KanbanEmptyState from './kanban-empty-state.tsx'
import KanbanOnboarding from './kanban-onboarding.tsx'
import TicketCreateModal from './ticket-create-modal.tsx'
import TicketModal from './ticket-modal-new.tsx'

type KanbanBoardProps = {
  auditId: string
  teamId: string
}

const KanbanBoard = ({ auditId, teamId }: KanbanBoardProps) => {
  const { t } = useTranslation()
  const { data: board, isLoading, refetch } = useKanbanBoard(auditId)
  const { mutate: moveTicket } = useMoveTicket(auditId)
  const {
    role,
    isOwner,
    canManage,
    isLoading: isLoadingRole
  } = useAuditRole(teamId)

  const [activeTicket, setActiveTicket] = useState<TicketSummary | null>(null)
  const [selectedTicketId, setSelectedTicketId] = useState<string | null>(null)
  const {
    value: createModalOpen,
    toggle: toggleCreateModal,
    close: closeCreateModal
  } = useToggle(false)
  const [selectedColumnId, setSelectedColumnId] = useState<string | null>(null)

  const permissions = role ? getKanbanPermissions(role) : null

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8
      }
    })
  )

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event
    const ticket = findTicket(active.id as string)
    if (ticket) {
      setActiveTicket(ticket)
    }
  }

  const handleDragEnd = (event: DragEndEvent) => {
    const { active, over } = event
    setActiveTicket(null)

    if (!over) return

    const ticketId = active.id as string
    const overId = over.id as string

    const targetColumn = board?.columns.find(
      col => col.id === overId || col.tickets.some(t => t.id === overId)
    )

    if (!targetColumn) return

    const sourceTicket = findTicket(ticketId)
    if (!sourceTicket) return

    const sourceColumnId = sourceTicket.columnId
    const targetColumnId = targetColumn.id

    let newPosition: number
    if (overId === targetColumnId) {
      newPosition = targetColumn.tickets.length
    } else {
      const overTicketIndex = targetColumn.tickets.findIndex(
        t => t.id === overId
      )
      newPosition = Math.max(overTicketIndex, 0)
    }

    if (
      sourceColumnId !== targetColumnId ||
      sourceTicket.position !== newPosition
    ) {
      moveTicket({
        ticketId,
        payload: {
          columnId: targetColumnId,
          position: newPosition
        }
      })
    }
  }

  const findTicket = (ticketId: string): TicketSummary | null => {
    if (!board) return null
    for (const column of board.columns) {
      const ticket = column.tickets.find(t => t.id === ticketId)
      if (ticket) return ticket
    }
    return null
  }

  const handleAddTicket = (columnId: string) => {
    setSelectedColumnId(columnId)
    toggleCreateModal()
  }

  const handleTicketClick = (ticketId: string) => {
    setSelectedTicketId(ticketId)
  }

  const handleCloseCreateModal = () => {
    closeCreateModal()
    setSelectedColumnId(null)
  }

  const { data: teams } = useMyTeam()
  const availableUsers: TicketUser[] =
    teams
      ?.find(t => t.id === teamId)
      ?.members?.map(m => ({
        id: m.id,
        username: m.username,
        firstName: m.firstName,
        lastName: m.lastName,
        avatarUrl: m.avatarUrl ?? null
      })) ?? []

  if (isLoading || isLoadingRole) {
    return (
      <div className='flex h-full w-full items-center justify-center'>
        <Loader2 className='h-8 w-8 animate-spin text-orange-500' />
      </div>
    )
  }

  if (!board) {
    return (
      <div className='flex h-full w-full items-center justify-center p-8 text-center'>
        <p className='text-gray-500'>{t('kanban:error.loadFailed')}</p>
      </div>
    )
  }

  if (board.columns.length === 0) {
    if (isOwner || canManage) {
      return (
        <KanbanOnboarding
          auditId={auditId}
          onComplete={() => {
            refetch()
          }}
        />
      )
    } else {
      return <KanbanEmptyState />
    }
  }

  if (!permissions) {
    return (
      <div className='flex h-full w-full items-center justify-center p-8 text-center'>
        <p className='text-gray-500'>{t('kanban:error.noPermission')}</p>
      </div>
    )
  }

  return (
    <div className='flex h-full w-full min-w-0 flex-col'>
      <DndContext
        sensors={sensors}
        collisionDetection={closestCorners}
        onDragStart={permissions.canMoveTickets ? handleDragStart : undefined}
        onDragEnd={permissions.canMoveTickets ? handleDragEnd : undefined}
      >
        <div className='mb-4 flex items-center justify-between'>
          <h2 className='text-xl font-bold text-gray-900 dark:text-gray-100'>
            {t('kanban:board')}
          </h2>
        </div>

        <div className='flex flex-1 gap-4 overflow-x-auto pb-4'>
          {board.columns.map(column => (
            <KanbanColumn
              key={column.id}
              column={column}
              onAddTicket={handleAddTicket}
              onTicketClick={handleTicketClick}
              canAddTicket={permissions.canCreateTickets}
              canDragTickets={permissions.canMoveTickets}
            />
          ))}
        </div>

        <DragOverlay>
          {activeTicket ? (
            <div className='rotate-3 opacity-80'>
              <KanbanCard ticket={activeTicket} onClick={() => {}} />
            </div>
          ) : null}
        </DragOverlay>
      </DndContext>

      {createModalOpen && selectedColumnId && (
        <TicketCreateModal
          isOpen={createModalOpen}
          onClose={handleCloseCreateModal}
          auditId={auditId}
          columnId={selectedColumnId}
          availableUsers={availableUsers}
        />
      )}

      {selectedTicketId && (
        <TicketModal
          isOpen={!!selectedTicketId}
          onClose={() => setSelectedTicketId(null)}
          auditId={auditId}
          ticketId={selectedTicketId}
          teamId={teamId}
          availableUsers={availableUsers}
          availableLabels={board.labels}
        />
      )}
    </div>
  )
}

export default KanbanBoard
