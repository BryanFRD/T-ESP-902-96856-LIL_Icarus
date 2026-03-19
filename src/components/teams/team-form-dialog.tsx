import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger
} from '@/components/ui/dialog'
import type { Team } from '@/domain/types/team.ts'

import TeamForm from './team-form'

interface Props {
  team?: Team
}

export function TeamFormDialog({
  team,
  children
}: React.PropsWithChildren<Props>) {
  const { t } = useTranslation('team')

  const [isOpen, setIsOpen] = useState<boolean>(false)

  const title = team
    ? t('shared.teamForm.title.edit')
    : t('shared.teamForm.title.create')

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>{children}</DialogTrigger>

      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
        </DialogHeader>

        <TeamForm team={team} onSuccess={() => setIsOpen(false)} />
      </DialogContent>
    </Dialog>
  )
}
