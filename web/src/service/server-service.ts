import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type { Server } from '@/domain/types/server.ts'

export const status = async () => {
  const res = await getAxiosClient().get<Server>('/status')
  return res.data
}
