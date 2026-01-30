import { useQuery } from '@tanstack/react-query'

import { getUser } from '@/service/userService'

export const useUser = (id: string) => {
  return useQuery({
    queryKey: ['user', id],
    queryFn: () => getUser(id)
  })
}
