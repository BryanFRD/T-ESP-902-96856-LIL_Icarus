import { useQuery } from '@tanstack/react-query'

import type { BoardResponse } from '@/domain/types/kanban.ts'
import { getKanbanBoard } from '@/service/kanban-service.ts'

export const useKanbanBoard = (auditId: string) => {
  return useQuery<BoardResponse, Error>({
    queryKey: ['kanban', 'board', auditId],
    queryFn: () => getKanbanBoard(auditId),
    enabled: !!auditId
  })
}
