import { useQuery } from '@tanstack/react-query'

import { getTeamCompanies } from '@/service/team-service.ts'

export const useTeamCompany = (teamId: string) => {
  return useQuery({
    queryKey: ['team', teamId, 'companies'],
    queryFn: () => getTeamCompanies(teamId),
    enabled: !!teamId
  })
}
