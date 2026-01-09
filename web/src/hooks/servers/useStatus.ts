import { useQuery } from '@tanstack/react-query'

import type { ServerStatus } from '@/domain/commonTypes/serverType.ts'
import { status } from '@/services/serverService'

export const useStatus = () => {
  return useQuery<ServerStatus, Error>({
    queryKey: ['status'],
    queryFn: status,
    staleTime: 0
  })
}
