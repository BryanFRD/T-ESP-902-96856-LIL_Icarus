import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type { Agent } from '@/domain/types/agent.ts'

export const getAgents = async (auditId: string) => {
  const res = await getAxiosClient().get<Agent[]>(`/audits/${auditId}/agents`)
  return res.data
}

export const revokeAgent = async (auditId: string, agentId: string) => {
  await getAxiosClient().delete(`/audits/${auditId}/agents/${agentId}`)
}

export const toggleKeylogger = async (
  auditId: string,
  agentId: string,
  action: 'start' | 'stop'
) => {
  await getAxiosClient().post(
    `/audits/${auditId}/agents/${agentId}/keylogger`,
    { action }
  )
}
