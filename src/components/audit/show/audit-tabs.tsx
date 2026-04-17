import { useTranslation } from 'react-i18next'

import KanbanBoard from '@/components/kanban/kanban-board.tsx'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { UserAudit } from '@/domain/types/audit.ts'

import { AuditAgentsContent } from './agents/audit-agents-content'
import { AuditClientsContent } from './clients/audit-clients-content'
import { AuditExploitsContent } from './exploits/audit-exploits-content'
import { AuditInvitationsContent } from './invitations/audit-invitations-content'
import { AuditLogsContent } from './logs/audit-logs-content'
import { AuditMessagesContent } from './messages/audit-messages-content'

interface AuditTabsProps {
  audit: UserAudit
}

export function AuditTabs({ audit }: AuditTabsProps) {
  const { t } = useTranslation('audit')

  const tabs = [
    { value: 'kanban', label: t('show.tabs.kanban') },
    { value: 'clients', label: t('show.tabs.clients') },
    { value: 'invitations', label: t('show.tabs.invitations') },
    { value: 'messages', label: t('show.tabs.messages') },
    { value: 'agents', label: t('show.tabs.agents') },
    { value: 'exploits', label: t('show.tabs.exploits') },
    { value: 'logs', label: t('show.tabs.logs') }
  ]

  return (
    <Tabs defaultValue='kanban' className='w-full min-w-0 overflow-x-hidden'>
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

      <TabsContent value='messages'>
        <AuditMessagesContent auditId={audit.id} />
      </TabsContent>

      <TabsContent value='agents'>
        <AuditAgentsContent auditId={audit.id} />
      </TabsContent>

      <TabsContent value='exploits'>
        <AuditExploitsContent auditId={audit.id} />
      </TabsContent>

      <TabsContent value='logs'>
        <AuditLogsContent auditId={audit.id} />
      </TabsContent>
    </Tabs>
  )
}
