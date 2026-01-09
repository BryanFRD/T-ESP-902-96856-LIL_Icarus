import { useQuery } from '@tanstack/react-query'

import { getAvatar } from '@/services/userService.ts'

export const useGetAvatar = (filename: string | undefined) => {
  return useQuery({
    queryKey: ['user', 'avatar', filename],
    queryFn: () => getAvatar(filename ?? ''),
    enabled: !!filename,
    staleTime: 5 * 60 * 1000
  })
}
