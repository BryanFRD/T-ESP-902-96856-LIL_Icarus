import { useQuery } from '@tanstack/react-query'

import type { AuditCheckAccess } from '@/domain/types/audit.ts'
import { checkAuditsAccess } from '@/service/audit-service.ts'

export const useCheckAuditAccess = () => {
  return useQuery<AuditCheckAccess, Error>({
    queryKey: ['audits', 'check-access'],
    queryFn: checkAuditsAccess
  })
}
