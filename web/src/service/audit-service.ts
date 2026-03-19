import { HttpStatusCode } from 'axios'

import { getAxiosClient } from '@/configuration/api/axios-client.ts'
import type { AuditCheckAccess, UserAudit } from '@/domain/types/audit.ts'
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
