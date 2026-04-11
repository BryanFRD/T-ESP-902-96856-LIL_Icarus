import type { Company } from '@/domain/types/company.ts'

export type AuditMessageAuthor = {
  id: string
  username: string
  firstName: string
  lastName: string
  avatarUrl?: string
}

export type AuditMessage = {
  id: string
  auditId: string
  author: AuditMessageAuthor
  content: string
  createdAt: string
  updatedAt: string
}

export type PaginatedAuditMessages = {
  data: AuditMessage[]
  total: number
  limit: number
  offset: number
}

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

export type AuditLogUser = {
  id: string
  username: string
  email: string
  firstName: string
  lastName: string
  avatarUrl?: string
}

export type AuditLogEntry = {
  id: string
  userId?: string
  user?: AuditLogUser
  action: string
  resourceType: string
  resourceId?: string
  requestMethod: string
  requestPath: string
  metadata?: Record<string, unknown>
  changes?: Record<string, unknown>
  createdAt: string
}

export type PaginatedAuditLogs = {
  data: AuditLogEntry[]
  total: number
  limit: number
  offset: number
}
