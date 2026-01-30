import { useQuery } from '@tanstack/react-query'

import { getUsers } from '@/service/userService'

export const useUsers = () => {
  return useQuery({
    queryKey: ['user'],
    queryFn: getUsers
  })
}
