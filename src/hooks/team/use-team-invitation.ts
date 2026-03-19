import { useQuery } from '@tanstack/react-query'

import { getTeamInvitations } from '@/service/team-service.ts'

export const useTeamInvitation = (teamId: string) => {
  return useQuery({
    queryKey: ['team', teamId, 'invitations'],
    queryFn: () => getTeamInvitations(teamId),
    enabled: !!teamId
  })
}
