export const UserRole = {
  CLIENT: 'Client',
  PENTESTER: 'Pentester',
  ADMIN: 'Admin',
  SUPER_ADMIN: 'SuperAdmin'
} as const

export type UserRole = (typeof UserRole)[keyof typeof UserRole]
