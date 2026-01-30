import { useQuery } from '@tanstack/react-query'

import type { UserAudit } from '@/domain/types/Audit.ts'
import { getUserAudits } from '@/service/auditService.ts'

export const useUserAudits = () => {
  return useQuery<UserAudit[], Error>({
    queryKey: ['user', 'me', 'audits'],
    queryFn: getUserAudits
  })
}
