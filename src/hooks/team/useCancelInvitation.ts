import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Invitation } from '@/domain/types/Team.ts'
import { cancelInvitation } from '@/service/teamService'

export const useCancelInvitation = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: cancelInvitation,
    onSuccess: (_, variables: { teamId: string; invitationId: string }) => {
      const { teamId, invitationId } = variables
      queryClient.invalidateQueries({
        queryKey: ['team', teamId, 'invitations']
      })
      queryClient.setQueryData(
        ['team', teamId, 'invitations'],
        (oldData: Invitation[] | undefined) => {
          if (oldData) {
            return oldData.filter(invitation => invitation.id !== invitationId)
          }
          return oldData
        }
      )
    }
  })
}
