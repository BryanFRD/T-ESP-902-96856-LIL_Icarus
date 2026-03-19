import { useQuery } from '@tanstack/react-query'

import { getUsers } from '@/service/user-service.ts'

export const useUsers = () => {
  return useQuery({
    queryKey: ['user'],
    queryFn: getUsers
  })
}
