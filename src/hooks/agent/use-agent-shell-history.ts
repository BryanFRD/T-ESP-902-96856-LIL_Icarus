import { useQuery } from '@tanstack/react-query'

import { getAgentShellHistory } from '@/service/agent-service.ts'

export const useAgentShellHistory = (
  auditId: string,
  agentId: string,
  enabled: boolean
) =>
  useQuery({
    queryKey: ['agent-shell-history', auditId, agentId],
    queryFn: () => getAgentShellHistory(auditId, agentId),
    enabled,
    staleTime: Infinity,
    refetchOnWindowFocus: false,
    refetchOnMount: 'always'
  })
