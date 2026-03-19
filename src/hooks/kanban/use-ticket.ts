import { useQuery } from '@tanstack/react-query'

import type { TicketResponse } from '@/domain/types/kanban.ts'
import { getTicket } from '@/service/kanban-service.ts'

export const useTicket = (auditId: string, ticketId: string) => {
  return useQuery<TicketResponse, Error>({
    queryKey: ['kanban', 'ticket', auditId, ticketId],
    queryFn: () => getTicket(auditId, ticketId),
    enabled: !!auditId && !!ticketId
  })
}
