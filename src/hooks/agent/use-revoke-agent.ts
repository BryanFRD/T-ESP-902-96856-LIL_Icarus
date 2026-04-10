import { useMutation, useQueryClient } from '@tanstack/react-query'
import toast from 'react-hot-toast'
import { useTranslation } from 'react-i18next'

import { revokeAgent } from '@/service/agent-service.ts'
import { getErrorMessage } from '@/utils/api-utils.ts'

export const useRevokeAgent = (auditId: string) => {
  const queryClient = useQueryClient()
  const { t } = useTranslation('audit')

  return useMutation({
    mutationFn: (agentId: string) => revokeAgent(auditId, agentId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['agents', auditId] })
      toast.success(t('show.agents.toast.revoked'))
    },
    onError: error => {
      const message = getErrorMessage(
        error,
        'audit:show.agents.toast.revokeError'
      )
      toast.error(message)
    }
  })
}
