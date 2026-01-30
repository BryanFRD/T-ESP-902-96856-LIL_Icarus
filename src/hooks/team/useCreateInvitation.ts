import { useMutation, useQueryClient } from '@tanstack/react-query'

import type {
  CreateInvitationPayload,
  Invitation
} from '@/domain/types/Team.ts'
import { createInvitation } from '@/service/teamService'

export const useCreateInvitation = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createInvitation,
    onSuccess: (
      newInvitation: Invitation,
      variables: { teamId: string; payload: CreateInvitationPayload }
    ) => {
      const { teamId } = variables
      queryClient.invalidateQueries({
        queryKey: ['team', teamId, 'invitations']
      })
      queryClient.setQueryData(
        ['team', teamId, 'invitations'],
        (oldData: Invitation[] | undefined) => {
          if (oldData) {
            return [...oldData, newInvitation]
          }
          return [newInvitation]
        }
      )
    }
  })
}
