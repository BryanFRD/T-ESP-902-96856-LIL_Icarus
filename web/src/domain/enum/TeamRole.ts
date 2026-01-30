export const TeamRole = {
  Owner: 'Owner',
  Admin: 'Admin',
  Member: 'Member',
  Moderator: 'Moderator'
} as const

export type TeamRolesEnum = (typeof TeamRole)[keyof typeof TeamRole]
