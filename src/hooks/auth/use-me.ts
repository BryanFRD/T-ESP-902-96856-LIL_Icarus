import { useQuery } from '@tanstack/react-query'

import { me } from '@/service/auth-service.ts'

export const useMe = (ready: boolean = true) =>
  useQuery({
    queryKey: ['user', 'me'],
    queryFn: me,
    enabled: ready,
    staleTime: 5 * 60 * 1000
  })
