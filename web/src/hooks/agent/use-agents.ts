import { useQuery } from '@tanstack/react-query'

import type { Agent } from '@/domain/types/agent.ts'
import { getAgents } from '@/service/agent-service.ts'

export const useAgents = (auditId: string) => {
  return useQuery<Agent[], Error>({
    queryKey: ['agents', auditId],
    queryFn: () => getAgents(auditId),
    refetchInterval: 30_000
  })
}
