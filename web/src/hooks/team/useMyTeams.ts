import { useQuery } from '@tanstack/react-query'

import { getMyTeams } from '@/service/teamService'

export const useMyTeams = () => {
  return useQuery({
    queryKey: ['team', 'me'],
    queryFn: getMyTeams
  })
}
