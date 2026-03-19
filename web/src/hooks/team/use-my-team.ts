import { useQuery } from '@tanstack/react-query'

import { getMyTeams } from '@/service/team-service.ts'

export const useMyTeam = () => {
  return useQuery({
    queryKey: ['team', 'me'],
    queryFn: getMyTeams
  })
}
