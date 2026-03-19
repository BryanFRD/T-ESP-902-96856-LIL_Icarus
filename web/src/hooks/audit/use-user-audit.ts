import { useQuery } from '@tanstack/react-query'

import type { UserAudit } from '@/domain/types/audit.ts'
import { getUserAudits } from '@/service/audit-service.ts'

export const useUserAudit = () => {
  return useQuery<UserAudit[], Error>({
    queryKey: ['user', 'me', 'audits'],
    queryFn: getUserAudits
  })
}
