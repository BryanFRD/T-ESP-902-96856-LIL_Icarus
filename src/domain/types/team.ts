import type { MemberRole } from '../enum/member-role.ts'
import type { UserRole } from '../enum/role-enum.ts'

export type Team = {
  id: string
  name: string
  description?: string
  createdAt: string
  updatedAt: string
  deletedAt?: string
  members?: TeamMember[]
}

// TeamMember is a User object with additional role fields
export type TeamMember = {
  id: string
  username: string
  firstName: string
  lastName: string
  avatarUrl?: string
  email: string
  userRole: UserRole
  teamRole: MemberRole
  createdAt: string
  updatedAt: string
  deletedAt?: string
}

// Full team response including members and companies
export type TeamWithDetails = Team & {
  members?: TeamMember[]
  companies?: Company[]
}

export type Company = {
  id: string
  teamId: string
  name: string
  siret: string
  description?: string
  createdAt: string
  updatedAt: string
  deletedAt?: string
}

export type Invitation = {
  id: string
  teamId: string
  token: string
  maxUses: number
  usedCount: number
  isCanceled: boolean
  expiresAt?: string
  createdAt: string
  updatedAt: string
}

export type CreateTeamPayload = {
  name: string
  description?: string
}

export type PatchTeamPayload = {
  name?: string
  description?: string
}

export type CreateCompanyPayload = {
  name: string
  siret: string
  description?: string
}

export type PatchCompanyPayload = {
  name?: string
  siret?: string
  description?: string
}

export type CreateInvitationPayload = {
  maxUses: number
  expiresAt: string | null
}

export type UpdateMemberRolePayload = {
  role: MemberRole
}
