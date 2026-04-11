import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type { UserDashboardMetrics } from '@/domain/types/dashboard.ts'

export const getUserMetrics = async () => {
  return getAxiosClient()
    .get<UserDashboardMetrics>('/users/me/metrics')
    .then(res => res.data)
}
