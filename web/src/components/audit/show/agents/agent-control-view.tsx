import {
  ArrowLeftIcon,
  InfoIcon,
  KeyboardIcon,
  TerminalIcon
} from 'lucide-react'
import { useTranslation } from 'react-i18next'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import type { Agent } from '@/domain/types/agent.ts'
import { useAuditSocket } from '@/hooks/agent/use-audit-socket.ts'

import { InfoTab } from './info-tab'
import { KeyloggerTab } from './keylogger-tab'
import { ShellTab } from './shell-tab'

interface Props {
  agent: Agent
  auditId: string
  onBack: () => void
}

export function AgentControlView({ agent, auditId, onBack }: Props) {
  const { t } = useTranslation('audit')
  const socket = useAuditSocket(auditId)

  const statusBadge = agent.isRevoked ? (
    <Badge variant='destructive'>{t('show.agents.revoked')}</Badge>
  ) : agent.isOnline ? (
    <Badge variant='default'>{t('show.agents.online')}</Badge>
  ) : (
    <Badge variant='outline'>{t('show.agents.offline')}</Badge>
  )

  return (
    <div className='flex h-full flex-col'>
      <div className='flex items-center gap-3 pb-4'>
        <Button variant='ghost' size='icon-sm' onClick={onBack}>
          <ArrowLeftIcon className='size-4' />
        </Button>
        <h3 className='text-sm font-medium'>{agent.name}</h3>
        {statusBadge}
        {agent.isOnline && agent.latencyMs != null && (
          <span className='text-muted-foreground text-xs'>
            {agent.latencyMs} ms
          </span>
        )}
      </div>

      <Tabs
        defaultValue='shell'
        orientation='vertical'
        className='flex min-h-0 flex-1 flex-row gap-4'
      >
        <TabsList className='w-40 shrink-0'>
          <TabsTrigger value='shell' className='gap-2'>
            <TerminalIcon className='size-4' />
            {t('show.agents.controlPanel.tabs.shell')}
          </TabsTrigger>
          <TabsTrigger value='keylogger' className='gap-2'>
            <KeyboardIcon className='size-4' />
            {t('show.agents.controlPanel.tabs.keylogger')}
          </TabsTrigger>
          <TabsTrigger value='info' className='gap-2'>
            <InfoIcon className='size-4' />
            {t('show.agents.controlPanel.tabs.info')}
          </TabsTrigger>
        </TabsList>

        <TabsContent value='shell' className='min-h-0 flex-1'>
          <ShellTab socket={socket} agent={agent} auditId={auditId} />
        </TabsContent>
        <TabsContent value='keylogger' className='min-h-0 flex-1'>
          <KeyloggerTab socket={socket} agent={agent} auditId={auditId} />
        </TabsContent>
        <TabsContent value='info' className='min-h-0 flex-1'>
          <InfoTab agent={agent} auditId={auditId} />
        </TabsContent>
      </Tabs>
    </div>
  )
}
