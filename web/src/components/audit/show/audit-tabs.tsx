import { useTranslation } from 'react-i18next'

import KanbanBoard from '@/components/kanban/kanban-board.tsx'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { UserAudit } from '@/domain/types/audit.ts'

import { AuditClientsContent } from './clients/audit-clients-content'
import { AuditInvitationsContent } from './invitations/audit-invitations-content'

interface AuditTabsProps {
  audit: UserAudit
}

export function AuditTabs({ audit }: AuditTabsProps) {
  const { t } = useTranslation('audit')

  const tabs = [
    { value: 'kanban', label: t('show.tabs.kanban') },
    { value: 'clients', label: t('show.tabs.clients') },
    { value: 'invitations', label: t('show.tabs.invitations') }
  ]

  return (
    <Tabs defaultValue='kanban' className='w-full'>
      <TabsList className='w-full'>
        {tabs.map(tab => (
          <TabsTrigger key={tab.value} value={tab.value}>
            {tab.label}
          </TabsTrigger>
        ))}
      </TabsList>

      <TabsContent value='kanban'>
        <KanbanBoard auditId={audit.id} teamId={audit.teamId} />
      </TabsContent>

      <TabsContent value='clients'>
        <AuditClientsContent clients={audit.clients} />
      </TabsContent>

      <TabsContent value='invitations'>
        <AuditInvitationsContent />
      </TabsContent>
    </Tabs>
  )
}
