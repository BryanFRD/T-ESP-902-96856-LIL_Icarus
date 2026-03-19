import type { Company } from '@/domain/types/company.ts'

export type AuditCheckAccess = {
  hasAccess: boolean
  expiresAt: string
}

export type UserAudit = {
  clients: Client[]
  company: Company
  createdAt: string
  description: string
  endDate: string
  id: string
  isArchived: boolean
  name: string
  teamId: string
  updatedAt: string
}

export type Client = {
  id: string
  email: string
  firstName: string
  lastName: string
  username: string
  avatarUrl: string
}
