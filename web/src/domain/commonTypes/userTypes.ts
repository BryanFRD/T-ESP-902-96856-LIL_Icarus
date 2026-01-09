import type { UserRole } from '../enums/roles'

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
}
