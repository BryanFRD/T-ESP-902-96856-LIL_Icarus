import { getAxiosClient } from '@/configuration/api/axiosClient'
import type { Server } from '@/domain/types/Server.ts'

export const status = async () => {
  const res = await getAxiosClient().get<Server>('/status')
  return res.data
}
