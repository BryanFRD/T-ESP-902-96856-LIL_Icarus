import type { UserRole } from '@/domain/enum/role-enum.ts'
import type { TeamRolesEnum } from '@/domain/enum/team-role.ts'

export type User = {
  id: string
  username: string
  firstName: string
  lastName: string
  avatarUrl?: string
  email: string
  role: UserRole
  createdAt: string
  updatedAt: string
  deletedAt?: string
  teamRole?: TeamRolesEnum
}
