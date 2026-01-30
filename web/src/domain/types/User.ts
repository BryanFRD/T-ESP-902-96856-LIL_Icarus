import type { UserRole } from '@/domain/enum/RoleEnum.ts'
import type { TeamRolesEnum } from '@/domain/enum/TeamRole.ts'

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
