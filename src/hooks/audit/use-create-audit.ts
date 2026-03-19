import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { CreateAuditSchemaType } from '@/domain/validation/audit-schema.ts'
import { createAudit } from '@/service/audit-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

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
