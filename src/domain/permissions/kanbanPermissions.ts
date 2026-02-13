import { MemberRole } from '@/domain/enum/MemberRole'

export type KanbanPermissions = {
  canManageColumns: boolean
  canManageLabels: boolean
  canCreateTickets: boolean
  canEditAllTickets: boolean
  canDeleteAllTickets: boolean
  canMoveTickets: boolean
  canEditOwnComments: boolean
  canCreateComments: boolean
}

export const getKanbanPermissions = (role: MemberRole): KanbanPermissions => {
  const isOwnerOrModerator =
    role === MemberRole.OWNER || role === MemberRole.MODERATOR
  const isMember = role === MemberRole.MEMBER

  return {
    canManageColumns: isOwnerOrModerator,
    canManageLabels: isOwnerOrModerator,
    canCreateTickets: isOwnerOrModerator || isMember,
    canEditAllTickets: isOwnerOrModerator || isMember,
    canDeleteAllTickets: isOwnerOrModerator || isMember,
    canMoveTickets: isOwnerOrModerator || isMember,
    canEditOwnComments: true,
    canCreateComments: true
  }
}

export const canEditTicket = (role: MemberRole): boolean => {
  if (role === MemberRole.OWNER || role === MemberRole.MODERATOR) {
    return true
  }
  if (role === MemberRole.MEMBER) {
    return true
  }
  return false
}

export const canDeleteTicket = (role: MemberRole): boolean => {
  if (role === MemberRole.OWNER || role === MemberRole.MODERATOR) {
    return true
  }
  if (role === MemberRole.MEMBER) {
    return true
  }
  return false
}

export const canEditComment = (
  userId: string,
  commentAuthorId: string
): boolean => {
  return userId === commentAuthorId
}

export const canDeleteComment = (
  userId: string,
  commentAuthorId: string
): boolean => {
  return userId === commentAuthorId
}
