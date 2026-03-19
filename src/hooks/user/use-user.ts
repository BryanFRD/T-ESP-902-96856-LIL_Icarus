import { useQuery } from '@tanstack/react-query'

import { getUser } from '@/service/user-service.ts'

export const useUser = (id: string) => {
  return useQuery({
    queryKey: ['user', id],
    queryFn: () => getUser(id)
  })
}
