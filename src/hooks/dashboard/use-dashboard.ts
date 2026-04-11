import { useQuery } from '@tanstack/react-query'

import { getUserMetrics } from '@/service/dashboard-service.ts'

export const useDashboard = () => {
  return useQuery({
    queryKey: ['user-metrics'],
    queryFn: getUserMetrics,
    refetchInterval: 30_000
  })
}
