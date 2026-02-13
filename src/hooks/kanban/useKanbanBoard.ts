import { useQuery } from '@tanstack/react-query'

import type { BoardResponse } from '@/domain/types/Kanban'
import { getKanbanBoard } from '@/service/kanbanService'

export const useKanbanBoard = (auditId: string) => {
  return useQuery<BoardResponse, Error>({
    queryKey: ['kanban', 'board', auditId],
    queryFn: () => getKanbanBoard(auditId),
    enabled: !!auditId
  })
}
