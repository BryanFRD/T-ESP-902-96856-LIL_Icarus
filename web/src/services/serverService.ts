import { getAxiosClient } from '@/api/axiosClient'
import type { ServerStatus } from '@/domain/commonTypes/serverType'

export const status = async () => {
  const res = await getAxiosClient().get<ServerStatus>('/status')
  return res.data
}
