import { UserMinusIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { toast } from 'sonner'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogMedia,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import type { TeamMember } from '@/domain/types/team.ts'
import { useRemoveMember } from '@/hooks/team/use-remove-member.ts'

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  teamId: string
  member: TeamMember
}

export function RemoveMemberDialog({
  open,
  onOpenChange,
  teamId,
  member
}: Props) {
  const { t } = useTranslation('team')
  const { mutate, isPending } = useRemoveMember()

  const displayName =
    member.firstName && member.lastName
      ? `${member.firstName} ${member.lastName}`
      : member.username || member.email

  function handleConfirm() {
    mutate(
      { teamId, memberId: member.id },
      {
        onSuccess: () => {
          toast.success(t('show.members.removeMember.toast.success'))
          onOpenChange(false)
        },
        onError: () => {
          toast.error(t('show.members.removeMember.toast.error'))
        }
      }
    )
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogMedia className='size-12'>
            <UserMinusIcon className='size-6' />
          </AlertDialogMedia>

          <AlertDialogTitle>
            {t('show.members.removeMember.title')}
          </AlertDialogTitle>

          <AlertDialogDescription>
            {t('show.members.removeMember.description', { name: displayName })}
          </AlertDialogDescription>
        </AlertDialogHeader>

        <AlertDialogFooter>
          <AlertDialogCancel>
            {t('show.members.removeMember.cancel')}
          </AlertDialogCancel>

          <AlertDialogAction
            variant='destructive'
            disabled={isPending}
            onClick={handleConfirm}
          >
            {t('show.members.removeMember.confirm')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
