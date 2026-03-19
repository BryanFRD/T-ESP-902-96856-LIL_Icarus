import { useMutation, useQueryClient } from '@tanstack/react-query'

import { verifyAuditAccess } from '@/service/audit-service.ts'

export const useVerifyAuditAccess = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: (password: string) => verifyAuditAccess(password),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['audits', 'check-access'] })
    }
  })
}
