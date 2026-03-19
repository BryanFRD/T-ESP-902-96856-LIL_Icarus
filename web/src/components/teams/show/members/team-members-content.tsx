import { MemberRole } from '@/domain/enum/member-role.ts'
import type { TeamMember } from '@/domain/types/team.ts'
import { useMe } from '@/hooks/auth/use-me.ts'

import { MemberRow } from './team-member-row'

interface Props {
  teamId: string
  members?: TeamMember[]
}

export function TeamMembersContent({ teamId, members }: Props) {
  const { data: me } = useMe()

  const currentUserRole = members?.find(m => m.id === me?.id)?.teamRole

  const canManage =
    currentUserRole === MemberRole.OWNER ||
    currentUserRole === MemberRole.MODERATOR

  const sortedMembers = [...(members || [])].sort((a, b) => {
    const rolePriority = {
      OWNER: 1,
      MODERATOR: 2,
      MEMBER: 3,
      CLIENT: 4
    }

    return rolePriority[a.teamRole] - rolePriority[b.teamRole]
  })

  return (
    <div className='space-y-3'>
      {sortedMembers.map(member => (
        <MemberRow
          key={member.id}
          member={member}
          canManage={canManage}
          teamId={teamId}
        />
      ))}
    </div>
  )
}
