import { useTranslation } from 'react-i18next'

import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle
} from '@/components/ui/alert-dialog'
import type { Agent } from '@/domain/types/agent.ts'
import { useRevokeAgent } from '@/hooks/agent/use-revoke-agent.ts'

interface Props {
  open: boolean
  onOpenChange: (open: boolean) => void
  agent: Agent
  auditId: string
}

export function RevokeAgentDialog({
  open,
  onOpenChange,
  agent,
  auditId
}: Props) {
  const { t } = useTranslation('audit')
  const { mutate: revoke, isPending } = useRevokeAgent(auditId)

  const handleRevoke = () => {
    revoke(agent.id, {
      onSuccess: () => onOpenChange(false)
    })
  }

  return (
    <AlertDialog open={open} onOpenChange={onOpenChange}>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>
            {t('show.agents.revokeDialog.title')}
          </AlertDialogTitle>
          <AlertDialogDescription>
            {t('show.agents.revokeDialog.description', { name: agent.name })}
          </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
          <AlertDialogCancel>
            {t('show.agents.revokeDialog.cancel')}
          </AlertDialogCancel>
          <AlertDialogAction onClick={handleRevoke} disabled={isPending}>
            {t('show.agents.revokeDialog.confirm')}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  )
}
