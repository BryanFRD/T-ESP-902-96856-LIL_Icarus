import { PlusIcon } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import { MemberRole } from '@/domain/enum/member-role.ts'
import type { TeamMember } from '@/domain/types/team.ts'
import { useMe } from '@/hooks/auth/use-me.ts'
import { useTeamInvitation } from '@/hooks/team/use-team-invitation.ts'

import { CreateInvitationDialog } from './create-invitation-dialog'
import { InvitationRow } from './team-invitation-row'

interface Props {
  teamId: string
  members?: TeamMember[]
}

export function TeamInvitationsContent({ teamId, members }: Props) {
  const { t } = useTranslation('team')
  const { data: me } = useMe()
  const { data: invitations } = useTeamInvitation(teamId)

  const [createOpen, setCreateOpen] = useState(false)

  const currentUserRole = members?.find(m => m.id === me?.id)?.teamRole
  const canManage =
    currentUserRole === MemberRole.OWNER ||
    currentUserRole === MemberRole.MODERATOR

  const now = new Date()

  const safeInvitation = invitations ?? []

  const isDisabled = (inv: (typeof safeInvitation)[number]) =>
    inv.isCanceled ||
    inv.usedCount >= inv.maxUses ||
    (!!inv.expiresAt && new Date(inv.expiresAt) < now)

  const sortedInvitations = safeInvitation
    ? [...safeInvitation].sort((a, b) => {
        const byActive = Number(isDisabled(a)) - Number(isDisabled(b))
        if (byActive !== 0) return byActive

        const byCreatedAt =
          new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime()
        if (byCreatedAt !== 0) return byCreatedAt

        const byExpiresAt =
          new Date(a.expiresAt ?? 0).getTime() -
          new Date(b.expiresAt ?? 0).getTime()
        if (byExpiresAt !== 0) return byExpiresAt

        return Number(a.isCanceled) - Number(b.isCanceled)
      })
    : []

  return (
    <div className='space-y-3'>
      {canManage && (
        <div className='flex justify-end'>
          <Button size='sm' onClick={() => setCreateOpen(true)}>
            <PlusIcon className='size-4' />
            <span>{t('show.invitations.add')}</span>
          </Button>
        </div>
      )}

      {!sortedInvitations.length ? (
        <p className='text-muted-foreground py-4 text-center text-sm'>
          {t('show.invitations.empty')}
        </p>
      ) : (
        sortedInvitations.map(invitation => (
          <InvitationRow
            key={invitation.id}
            invitation={invitation}
            canManage={canManage}
            teamId={teamId}
          />
        ))
      )}

      <CreateInvitationDialog
        open={createOpen}
        onOpenChange={setCreateOpen}
        teamId={teamId}
      />
    </div>
  )
}
