import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Team } from '@/domain/types/Team.ts'
import { deleteTeam } from '@/service/teamService'

export const useDeleteTeam = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: deleteTeam,
    onSuccess: (_, teamId) => {
      queryClient.invalidateQueries({ queryKey: ['team', 'me'] })
      queryClient.invalidateQueries({ queryKey: ['team', teamId] })
      queryClient.setQueryData(
        ['team', 'me'],
        (oldData: Team[] | undefined) => {
          if (oldData) {
            return oldData.filter(team => team.id !== teamId)
          }
          return oldData
        }
      )
    }
  })
}
