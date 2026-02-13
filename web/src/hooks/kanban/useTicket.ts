import { useQuery } from '@tanstack/react-query'

import type { TicketResponse } from '@/domain/types/Kanban'
import { getTicket } from '@/service/kanbanService'

export const useTicket = (auditId: string, ticketId: string) => {
  return useQuery<TicketResponse, Error>({
    queryKey: ['kanban', 'ticket', auditId, ticketId],
    queryFn: () => getTicket(auditId, ticketId),
    enabled: !!auditId && !!ticketId
  })
}
