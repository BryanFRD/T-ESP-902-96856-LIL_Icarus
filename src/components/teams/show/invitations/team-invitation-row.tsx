import { ClockIcon, CopyIcon, Link2Icon, XIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import { Card } from '@/components/ui/card'
import type { Invitation } from '@/domain/types/team.ts'
import { useCancelInvitation } from '@/hooks/team/use-cancel-invitation.ts'
import { cn } from '@/lib/utils'
import { useDateFormatter } from '@/utils/date-formatter.ts'

interface Props {
  invitation: Invitation
  canManage: boolean
  teamId: string
}

export function InvitationRow({ invitation, canManage, teamId }: Props) {
  const { t } = useTranslation('team')
  const { dateLong } = useDateFormatter()
  const { mutate: cancelInvitation, isPending } = useCancelInvitation()

  const isExpired = invitation.expiresAt
    ? new Date(invitation.expiresAt) < new Date()
    : false
  const isFullyUsed = invitation.usedCount >= invitation.maxUses
  const isDisabled = invitation.isCanceled || isExpired || isFullyUsed

  const getStatusLabel = () => {
    if (invitation.isCanceled) return t('show.invitations.status.canceled')
    if (isFullyUsed) return t('show.invitations.status.fullyUsed')
    if (isExpired)
      return t('show.invitations.status.expiredAt', {
        date: dateLong(invitation.expiresAt!)
      })
    if (invitation.expiresAt)
      return t('show.invitations.status.expiresAt', {
        date: dateLong(invitation.expiresAt)
      })
    return t('show.invitations.status.noExpiration')
  }

  const handleCopy = () => {
    const link = `${window.location.origin}/teams/join/${invitation.token}`
    navigator.clipboard.writeText(link)
    toast.success(t('show.invitations.toast.copied'))
  }

  const handleCancel = () => {
    cancelInvitation(
      { teamId, invitationId: invitation.id },
      {
        onSuccess: () => toast.success(t('show.invitations.toast.canceled')),
        onError: () => toast.error(t('show.invitations.toast.cancelError'))
      }
    )
  }

  return (
    <Card
      className={cn(
        'flex-row items-center gap-x-2 px-4',
        isDisabled && 'opacity-60'
      )}
    >
      <div className='bg-muted flex size-9 shrink-0 items-center justify-center rounded-full'>
        <Link2Icon
          className={cn(
            'size-4',
            isDisabled ? 'text-muted-foreground' : 'text-primary'
          )}
        />
      </div>

      <div className='flex flex-1 flex-col'>
        <p className='text-sm font-medium'>
          {t('show.invitations.usageCount', {
            used: invitation.usedCount,
            max: invitation.maxUses
          })}
        </p>
        <p
          className={cn(
            'flex items-center gap-1 text-xs',
            isDisabled ? 'text-destructive' : 'text-muted-foreground'
          )}
        >
          <ClockIcon className='size-3' />
          {getStatusLabel()}
        </p>
      </div>

      <div className='flex items-center gap-1'>
        {!isDisabled && (
          <Button variant='ghost' size='icon' onClick={handleCopy}>
            <CopyIcon className='size-4' />
          </Button>
        )}

        {canManage && !invitation.isCanceled && (
          <Button
            variant='ghost'
            size='icon'
            onClick={handleCancel}
            disabled={isPending}
            className='text-destructive hover:text-destructive'
          >
            <XIcon className='size-4' />
          </Button>
        )}
      </div>
    </Card>
  )
}
