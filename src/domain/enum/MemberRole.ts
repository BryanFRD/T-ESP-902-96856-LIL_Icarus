export const MemberRole = {
  OWNER: 'OWNER',
  MODERATOR: 'MODERATOR',
  MEMBER: 'MEMBER',
  CLIENT: 'CLIENT'
} as const

export type MemberRole = (typeof MemberRole)[keyof typeof MemberRole]
