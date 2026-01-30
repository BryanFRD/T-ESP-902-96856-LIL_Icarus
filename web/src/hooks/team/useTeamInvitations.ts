import { useQuery } from '@tanstack/react-query'

import { getTeamInvitations } from '@/service/teamService'

export const useTeamInvitations = (teamId: string) => {
  return useQuery({
    queryKey: ['team', teamId, 'invitations'],
    queryFn: () => getTeamInvitations(teamId),
    enabled: !!teamId
  })
}
