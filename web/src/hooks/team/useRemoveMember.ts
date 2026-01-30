import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Team } from '@/domain/types/Team.ts'
import { removeMember } from '@/service/teamService'

export const useRemoveMember = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: removeMember,
    onSuccess: (_, variables: { teamId: string; memberId: string }) => {
      const { teamId, memberId } = variables
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
                  members: team.members.filter(member => member.id !== memberId)
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
