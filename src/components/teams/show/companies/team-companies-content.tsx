import { PlusIcon } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import { MemberRole } from '@/domain/enum/member-role.ts'
import type { TeamMember } from '@/domain/types/team.ts'
import { useMe } from '@/hooks/auth/use-me.ts'
import { useTeamCompany } from '@/hooks/team/use-team-company.ts'

import { CompanyFormDialog } from './company-form-dialog'
import { CompanyRow } from './team-company-row'

interface Props {
  teamId: string
  members?: TeamMember[]
}

export function TeamCompaniesContent({ teamId, members }: Props) {
  const { t } = useTranslation('team')
  const { data: me } = useMe()
  const { data: companies } = useTeamCompany(teamId)

  const [createOpen, setCreateOpen] = useState(false)

  const currentUserRole = members?.find(m => m.id === me?.id)?.teamRole
  const canManage =
    currentUserRole === MemberRole.OWNER ||
    currentUserRole === MemberRole.MODERATOR

  return (
    <div className='space-y-3'>
      {canManage && (
        <div className='flex justify-end'>
          <Button size='sm' onClick={() => setCreateOpen(true)}>
            <PlusIcon className='size-4' />
            {t('show.companies.add')}
          </Button>
        </div>
      )}

      {!companies?.length ? (
        <p className='text-muted-foreground py-4 text-center text-sm'>
          {t('show.companies.empty')}
        </p>
      ) : (
        companies.map(company => (
          <CompanyRow
            key={company.id}
            company={company}
            canManage={canManage}
            teamId={teamId}
          />
        ))
      )}

      <CompanyFormDialog
        open={createOpen}
        onOpenChange={setCreateOpen}
        teamId={teamId}
      />
    </div>
  )
}
