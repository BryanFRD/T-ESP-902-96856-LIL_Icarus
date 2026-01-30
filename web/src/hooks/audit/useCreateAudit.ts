import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { CreateAuditSchemaType } from '@/domain/validation/auditSchema.ts'
import { createAudit } from '@/service/auditService.ts'
import { getErrorMessage } from '@/utils/apiUtils.ts'

export const useCreateAudit = () => {
  const queryClient = useQueryClient()

  return useMutation<void, Error, CreateAuditSchemaType>({
    mutationFn: createAudit,
    onError: error => {
      return getErrorMessage(error, 'error.login.failed')
    },
    onSuccess: async () => {
      await queryClient.invalidateQueries({
        queryKey: ['user', 'me', 'audit']
      })
    }
  })
}
