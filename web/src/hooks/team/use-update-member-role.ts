import { useMutation, useQueryClient } from '@tanstack/react-query'

import type {
  Team,
  TeamMember,
  UpdateMemberRolePayload
} from '@/domain/types/team.ts'
import { updateMemberRole } from '@/service/team-service.ts'

export const useUpdateMemberRole = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: updateMemberRole,
    onSuccess: (
      updatedMember: TeamMember,
      variables: {
        teamId: string
        memberId: string
        payload: UpdateMemberRolePayload
      }
    ) => {
      const { teamId, memberId, payload } = variables
      queryClient.invalidateQueries({ queryKey: ['team', 'me'] })
      queryClient.invalidateQueries({ queryKey: ['team', teamId] })
      queryClient.setQueryData(
        ['team', 'me'],
        (oldData: Team[] | undefined) => {
          if (oldData) {
            return oldData.map(team => {
              if (team.id === teamId && team.members) {
                return {
                  ...team,
                  members: team.members.map(member =>
                    member.id === memberId
                      ? { ...member, ...payload, ...updatedMember }
                      : member
                  )
                }
              }
              return team
            })
          }
          return oldData
        }
      )
    }
  })
}
