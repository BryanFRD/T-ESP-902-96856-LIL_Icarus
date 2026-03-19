import { useMutation, useQueryClient } from '@tanstack/react-query'

import type { PatchTeamPayload, Team } from '@/domain/types/team.ts'
import { patchTeam } from '@/service/team-service.ts'

export const usePatchTeam = () => {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: patchTeam,
    onSuccess: (
      updatedTeam: Team,
      variables: { teamId: string; payload: PatchTeamPayload }
    ) => {
      const { teamId, payload } = variables
      queryClient.invalidateQueries({ queryKey: ['team', 'me'] })
      queryClient.invalidateQueries({ queryKey: ['team', teamId] })
      queryClient.setQueryData(
        ['team', teamId],
        (oldData: Team | undefined) => {
          if (oldData) {
            return {
              ...oldData,
              ...payload,
              ...updatedTeam
            }
          }
          return updatedTeam
        }
      )
      queryClient.setQueryData(
        ['team', 'me'],
        (oldData: Team[] | undefined) => {
          if (oldData) {
            return oldData.map(team =>
              team.id === teamId
                ? { ...team, ...payload, ...updatedTeam }
                : team
            )
          }
          return oldData
        }
      )
    }
  })
}
