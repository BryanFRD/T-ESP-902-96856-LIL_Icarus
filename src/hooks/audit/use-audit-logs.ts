import { useQuery } from '@tanstack/react-query'

import { getAuditLogs } from '@/service/audit-service.ts'

export const useAuditLogs = (
  auditId: string,
  limit = 50,
  offset = 0,
  action?: string
) => {
  return useQuery({
    queryKey: ['audit-logs', auditId, limit, offset, action],
    queryFn: () => getAuditLogs(auditId, limit, offset, action),
    refetchInterval: 30_000
  })
}
