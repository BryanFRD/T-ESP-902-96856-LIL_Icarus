import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { Team } from '@/domain/types/team.ts'
import { createTeam } from '@/service/team-service.ts'

export const useCreateTeam = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: createTeam,
    onSuccess: (newTeam: Team) => {
      queryClient.invalidateQueries({ queryKey: ['team', 'me'] })
      queryClient.setQueryData(
        ['team', 'me'],
        (oldData: Team[] | undefined) => {
          if (oldData) {
            return [...oldData, newTeam]
          }
          return [newTeam]
        }
      )
    }
  })
}
