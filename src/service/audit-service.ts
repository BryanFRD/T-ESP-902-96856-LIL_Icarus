import { HttpStatusCode } from 'axios'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type {
  AuditCheckAccess,
  AuditMessage,
  PaginatedAuditLogs,
  PaginatedAuditMessages,
  UserAudit
} from '@/domain/types/audit.ts'
import type { CreateAuditSchemaType } from '@/domain/validation/audit-schema.ts'

export const checkAuditsAccess = async () => {
  return getAxiosClient()
    .get<AuditCheckAccess>('/audits/check-access')
    .then(res => res.data)
}

export const verifyAuditAccess = async (password: string) => {
  return getAxiosClient()
    .post('/audits/verify-access', { password })
    .then(res => res.status === HttpStatusCode.Ok)
}

export const createAudit = async (data: CreateAuditSchemaType) => {
  return getAxiosClient()
    .post('/audits/me', data)
    .then(res => res.data)
}

export const getUserAudits = async () => {
  return getAxiosClient()
    .get<UserAudit[]>('/audits/me')
    .then(res => res.data)
}

export const getAuditMessages = async (
  auditId: string,
  limit = 100,
  offset = 0
) => {
  return getAxiosClient()
    .get<PaginatedAuditMessages>(`/audits/${auditId}/messages`, {
      params: { limit, offset }
    })
    .then(res => res.data)
}

export const sendAuditMessage = async (auditId: string, content: string) => {
  return getAxiosClient()
    .post<AuditMessage>(`/audits/${auditId}/messages`, { content })
    .then(res => res.data)
}

export const getAuditLogs = async (
  auditId: string,
  limit = 50,
  offset = 0,
  action?: string
) => {
  return getAxiosClient()
    .get<PaginatedAuditLogs>(`/audits/${auditId}/logs`, {
      params: { limit, offset, include_user: true, ...(action && { action }) }
    })
    .then(res => res.data)
}
