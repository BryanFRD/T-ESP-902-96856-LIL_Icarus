import { useMutation, useQueryClient } from '@tanstack/react-query'

import { joinTeam } from '@/service/team-service.ts'

export const useJoinTeam = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: joinTeam,
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['team', 'me'] })
    }
  })
}
