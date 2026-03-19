import {
  CrownIcon,
  EllipsisIcon,
  ShieldIcon,
  UserIcon,
  UserMinusIcon
} from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { MemberRole } from '@/domain/enum/member-role.ts'
import type { TeamMember } from '@/domain/types/team.ts'
import { useGetAvatar } from '@/hooks/user/use-get-avatar.ts'

import { RemoveMemberDialog } from './remove-member-dialog'
import { UpdateRoleUserDialog } from './update-role-user-dialog'

const roleConfig: Record<
  MemberRole,
  {
    icon: React.ElementType
    variant: 'default' | 'secondary' | 'outline'
  }
> = {
  [MemberRole.OWNER]: { icon: CrownIcon, variant: 'default' },
  [MemberRole.MODERATOR]: { icon: ShieldIcon, variant: 'secondary' },
  [MemberRole.MEMBER]: { icon: UserIcon, variant: 'outline' },
  [MemberRole.CLIENT]: { icon: UserIcon, variant: 'outline' }
}

type TeamRole = 'owner' | 'moderator' | 'member' | 'client'

interface Props {
  member: TeamMember
  canManage: boolean
  teamId: string
}

export function MemberRow({ member, canManage, teamId }: Props) {
  const { t } = useTranslation('team')
  const { data: avatarUrl } = useGetAvatar(member.avatarUrl)

  const [changeRoleOpen, setChangeRoleOpen] = useState(false)
  const [removeMemberOpen, setRemoveMemberOpen] = useState(false)

  const displayName =
    member.firstName && member.lastName
      ? `${member.firstName} ${member.lastName}`
      : member.username || member.email || t('show.members.unknownMember')

  const initials = displayName
    .split(' ')
    .map(n => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2)

  const { icon: RoleIcon, variant } = roleConfig[member.teamRole] ?? {
    icon: UserIcon,
    variant: 'outline' as const
  }

  return (
    <Card className='flex-row items-center gap-x-2 px-4'>
      <div className='flex flex-1 items-center gap-3'>
        <Avatar>
          <AvatarImage src={avatarUrl} alt={displayName} />
          <AvatarFallback>{initials}</AvatarFallback>
        </Avatar>

        <div>
          <p className='text-sm font-medium'>{displayName}</p>
          {member.email && (
            <p className='text-muted-foreground text-xs'>{member.email}</p>
          )}
        </div>
      </div>

      <Badge variant={variant}>
        <RoleIcon className='size-4' />
        {t(`memberRoles.${member.teamRole.toLowerCase() as TeamRole}`)}
      </Badge>

      {canManage && member.teamRole !== MemberRole.OWNER && (
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant='ghost' size='icon'>
              <EllipsisIcon className='size-4' />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent align='end'>
            <DropdownMenuItem onSelect={() => setChangeRoleOpen(true)}>
              <ShieldIcon className='size-4' />
              <span>{t('show.members.actions.changeRole')}</span>
            </DropdownMenuItem>

            <DropdownMenuItem
              variant='destructive'
              onSelect={() => setRemoveMemberOpen(true)}
            >
              <UserMinusIcon className='size-4' />
              <span>{t('show.members.actions.removeMember')}</span>
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      )}

      <UpdateRoleUserDialog
        open={changeRoleOpen}
        onOpenChange={setChangeRoleOpen}
        teamId={teamId}
        member={member}
      />

      <RemoveMemberDialog
        open={removeMemberOpen}
        onOpenChange={setRemoveMemberOpen}
        teamId={teamId}
        member={member}
      />
    </Card>
  )
}
