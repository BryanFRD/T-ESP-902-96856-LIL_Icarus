import { useMemo } from 'react'

import { useUserContext } from '@/context/user-context.tsx'
import { MemberRole } from '@/domain/enum/member-role.ts'
import { useMyTeam } from '@/hooks/team/use-my-team.ts'

export const useAuditRole = (teamId: string) => {
  const { user } = useUserContext()
  const { data: teams, isLoading } = useMyTeam()

  const userRole = useMemo(() => {
    if (!user || !teams || !teamId) {
      return null
    }

    const team = teams.find(t => t.id === teamId)
    if (!team || !team.members) {
      return null
    }

    const member = team.members.find(m => m.id === user.id)
    return member?.teamRole || null
  }, [user, teams, teamId])

  const isOwner = userRole === MemberRole.OWNER
  const isModerator = userRole === MemberRole.MODERATOR
  const isMember = userRole === MemberRole.MEMBER
  const isClient = userRole === MemberRole.CLIENT

  return {
    role: userRole,
    isOwner,
    isModerator,
    isMember,
    isClient,
    canManage: isOwner || isModerator,
    isLoading
  }
}
