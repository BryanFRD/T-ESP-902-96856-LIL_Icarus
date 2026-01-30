import { useQuery } from '@tanstack/react-query'

import { getTeamCompanies } from '@/service/teamService'

export const useTeamCompanies = (teamId: string) => {
  return useQuery({
    queryKey: ['team', teamId, 'companies'],
    queryFn: () => getTeamCompanies(teamId),
    enabled: !!teamId
  })
}
