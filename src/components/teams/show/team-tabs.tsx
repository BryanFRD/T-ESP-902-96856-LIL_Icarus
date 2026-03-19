import { useTranslation } from 'react-i18next'

import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { Team } from '@/domain/types/team.ts'

import { TeamCompaniesContent } from './companies/team-companies-content'
import { TeamInvitationsContent } from './invitations/team-invitations-content'
import { TeamMembersContent } from './members/team-members-content'

interface Props {
  team: Team
}

export function TeamTabs({ team }: Props) {
  const { t } = useTranslation('team')

  const tabs = [
    { value: 'members', label: t('show.tabs.members') },
    { value: 'companies', label: t('show.tabs.companies') },
    { value: 'invitations', label: t('show.tabs.invitations') }
  ]

  return (
    <Tabs defaultValue='members' className='w-full'>
      <TabsList className='w-full'>
        {tabs.map(tab => (
          <TabsTrigger key={tab.value} value={tab.value}>
            {tab.label}
          </TabsTrigger>
        ))}
      </TabsList>

      <TabsContent value='members'>
        <TeamMembersContent teamId={team.id} members={team.members} />
      </TabsContent>

      <TabsContent value='companies'>
        <TeamCompaniesContent teamId={team.id} members={team.members} />
      </TabsContent>

      <TabsContent value='invitations'>
        <TeamInvitationsContent teamId={team.id} members={team.members} />
      </TabsContent>
    </Tabs>
  )
}
