import { useQuery } from '@tanstack/react-query'

import type { AuditCheckAccess } from '@/domain/types/Audit.ts'
import { checkAuditsAccess } from '@/service/auditService.ts'

export const useCheckAuditsAccess = () => {
  return useQuery<AuditCheckAccess, Error>({
    queryKey: ['audits', 'check-access'],
    queryFn: checkAuditsAccess
  })
}
