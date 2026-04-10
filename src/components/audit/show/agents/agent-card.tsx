import { formatDistanceToNow } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import { BanIcon, MonitorIcon } from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import type { Agent } from '@/domain/types/agent.ts'

import { RevokeAgentDialog } from './revoke-agent-dialog'

const LOCALES: Record<string, typeof enUS> = { fr, en: enUS }

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(0)} KB`
  const mb = kb / 1024
  if (mb < 1024) return `${mb.toFixed(1)} MB`
  const gb = mb / 1024
  return `${gb.toFixed(1)} GB`
}

interface Props {
  agent: Agent
  auditId: string
  onSelect: () => void
}

export function AgentCard({ agent, auditId, onSelect }: Props) {
  const { t, i18n } = useTranslation('audit')
  const [revokeOpen, setRevokeOpen] = useState(false)
  const locale = LOCALES[i18n.language] ?? enUS

  const statusBadge = agent.isRevoked ? (
    <Badge variant='destructive'>{t('show.agents.revoked')}</Badge>
  ) : agent.isOnline ? (
    <Badge variant='default'>{t('show.agents.online')}</Badge>
  ) : (
    <Badge variant='outline'>{t('show.agents.offline')}</Badge>
  )

  const lastSeen = agent.lastConnectedAt
    ? t('show.agents.lastConnected', {
        time: formatDistanceToNow(new Date(agent.lastConnectedAt), { locale })
      })
    : t('show.agents.never')

  const systemSummary = agent.systemInfo
    ? `${agent.systemInfo.hostname} - ${agent.systemInfo.osName} - ${formatBytes(agent.systemInfo.totalMemoryBytes)} RAM`
    : null

  return (
    <>
      <div
        className='ring-foreground/10 hover:ring-foreground/20 bg-card flex cursor-pointer items-center gap-4 rounded-2xl p-4 ring-1 transition-shadow'
        onClick={onSelect}
      >
        <div className='bg-muted flex size-10 items-center justify-center rounded-full'>
          <MonitorIcon className='text-muted-foreground size-5' />
        </div>

        <div className='min-w-0 flex-1'>
          <p className='text-sm font-medium'>{agent.name}</p>
          {systemSummary && (
            <p className='text-muted-foreground truncate text-xs'>
              {systemSummary}
            </p>
          )}
          <p className='text-muted-foreground text-xs'>{lastSeen}</p>
        </div>

        <div className='flex items-center gap-2'>
          {agent.isOnline && agent.latencyMs != null && (
            <span className='text-muted-foreground text-xs'>
              {agent.latencyMs} ms
            </span>
          )}
          {statusBadge}
          {!agent.isRevoked && (
            <Button
              variant='ghost'
              size='icon-sm'
              onClick={e => {
                e.stopPropagation()
                setRevokeOpen(true)
              }}
              className='text-destructive hover:text-destructive'
            >
              <BanIcon className='size-4' />
            </Button>
          )}
        </div>
      </div>

      <RevokeAgentDialog
        open={revokeOpen}
        onOpenChange={setRevokeOpen}
        agent={agent}
        auditId={auditId}
      />
    </>
  )
}
