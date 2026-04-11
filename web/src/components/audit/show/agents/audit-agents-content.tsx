import { BotIcon, DownloadIcon } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'
import { Skeleton } from '@/components/ui/skeleton'
import type { Agent } from '@/domain/types/agent.ts'
import { useAgents } from '@/hooks/agent/use-agents.ts'

import { AgentCard } from './agent-card'
import { AgentControlView } from './agent-control-view'

interface Props {
  auditId: string
}

export function AuditAgentsContent({ auditId }: Props) {
  const { t } = useTranslation('audit')
  const { data: agents, isLoading } = useAgents(auditId)
  const [selectedAgent, setSelectedAgent] = useState<Agent | null>(null)

  const downloadUrl = `${__CONFIG__.ICARUS_API_URL}/v1/audits/${auditId}/agents/download`

  if (selectedAgent) {
    return (
      <AgentControlView
        agent={selectedAgent}
        auditId={auditId}
        onBack={() => setSelectedAgent(null)}
      />
    )
  }

  if (isLoading) {
    return (
      <div className='space-y-3'>
        <Skeleton className='h-10 w-full' />
        <Skeleton className='h-18 w-full rounded-2xl' />
        <Skeleton className='h-18 w-full rounded-2xl' />
      </div>
    )
  }

  return (
    <div className='space-y-4'>
      <div className='flex justify-end'>
        <Button variant='secondary' size='sm' asChild>
          <a href={downloadUrl} download>
            <DownloadIcon className='size-4' />
            {t('show.agents.download')}
          </a>
        </Button>
      </div>

      {agents && agents.length > 0 ? (
        <div className='space-y-3'>
          {[...agents]
            .sort((a, b) => {
              if (a.isOnline !== b.isOnline) return a.isOnline ? -1 : 1
              if (a.isRevoked !== b.isRevoked) return a.isRevoked ? 1 : -1
              return a.name.localeCompare(b.name)
            })
            .map(agent => (
              <AgentCard
                key={agent.id}
                agent={agent}
                auditId={auditId}
                onSelect={() => setSelectedAgent(agent)}
              />
            ))}
        </div>
      ) : (
        <Empty>
          <EmptyHeader>
            <EmptyMedia variant='icon'>
              <BotIcon />
            </EmptyMedia>
            <EmptyTitle>{t('show.agents.empty')}</EmptyTitle>
          </EmptyHeader>
        </Empty>
      )}
    </div>
  )
}
