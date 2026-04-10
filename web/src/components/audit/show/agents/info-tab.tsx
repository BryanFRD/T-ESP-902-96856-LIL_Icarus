import { formatDistanceToNow } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import { useTranslation } from 'react-i18next'

import { Badge } from '@/components/ui/badge'
import type { Agent } from '@/domain/types/agent.ts'

const LOCALES: Record<string, typeof enUS> = { fr, en: enUS }

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  const kb = bytes / 1024
  if (kb < 1024) return `${kb.toFixed(0)} KB`
  const mb = kb / 1024
  if (mb < 1024) return `${mb.toFixed(1)} MB`
  const gb = mb / 1024
  if (gb < 1024) return `${gb.toFixed(1)} GB`
  const tb = gb / 1024
  return `${tb.toFixed(2)} TB`
}

function formatUptime(secs: number): string {
  const days = Math.floor(secs / 86400)
  const hours = Math.floor((secs % 86400) / 3600)
  const minutes = Math.floor((secs % 3600) / 60)
  if (days > 0) return `${days}d ${hours}h ${minutes}m`
  if (hours > 0) return `${hours}h ${minutes}m`
  return `${minutes}m`
}

interface Props {
  agent: Agent
}

function InfoRow({ label, value }: { label: string; value: React.ReactNode }) {
  return (
    <div className='flex items-baseline justify-between gap-4 py-2'>
      <span className='text-muted-foreground shrink-0 text-sm'>{label}</span>
      <span className='text-right text-sm font-medium'>{value}</span>
    </div>
  )
}

function SectionTitle({ children }: { children: React.ReactNode }) {
  return (
    <h4 className='text-muted-foreground border-border mt-4 mb-2 border-b pb-1 text-xs font-semibold tracking-wide uppercase'>
      {children}
    </h4>
  )
}

export function InfoTab({ agent }: Props) {
  const { t, i18n } = useTranslation('audit')
  const locale = LOCALES[i18n.language] ?? enUS
  const si = agent.systemInfo

  if (!si) {
    return (
      <div className='text-muted-foreground flex h-full items-center justify-center text-sm'>
        {t('show.agents.controlPanel.info.noData')}
      </div>
    )
  }

  const p = 'show.agents.controlPanel.info'

  return (
    <div className='h-full space-y-1 overflow-y-auto p-4'>
      {/* Identity */}
      <SectionTitle>{t(`${p}.sections.identity`)}</SectionTitle>
      <InfoRow label={t(`${p}.hostname`)} value={si.hostname} />
      <InfoRow
        label={t(`${p}.username`)}
        value={
          <span className='inline-flex items-center gap-2'>
            {si.username}
            {si.isElevated && (
              <Badge variant='destructive' className='px-1.5 py-0 text-[10px]'>
                {t(`${p}.elevated`)}
              </Badge>
            )}
          </span>
        }
      />
      {si.domain && <InfoRow label={t(`${p}.domain`)} value={si.domain} />}
      <InfoRow label={t(`${p}.arch`)} value={si.arch} />
      {si.locale && <InfoRow label={t(`${p}.locale`)} value={si.locale} />}

      {/* System */}
      <SectionTitle>{t(`${p}.sections.system`)}</SectionTitle>
      <InfoRow label={t(`${p}.os`)} value={`${si.osName} ${si.osVersion}`} />
      <InfoRow
        label={t(`${p}.cpu`)}
        value={`${si.cpuName} (${si.cpuCount} cores)`}
      />
      {si.gpuName && <InfoRow label={t(`${p}.gpu`)} value={si.gpuName} />}
      <InfoRow
        label={t(`${p}.memory`)}
        value={`${formatBytes(si.usedMemoryBytes)} / ${formatBytes(si.totalMemoryBytes)}`}
      />
      <InfoRow
        label={t(`${p}.disk`)}
        value={`${formatBytes(si.usedDiskBytes)} / ${formatBytes(si.totalDiskBytes)}`}
      />
      <InfoRow label={t(`${p}.uptime`)} value={formatUptime(si.uptimeSecs)} />

      {/* Network */}
      <SectionTitle>{t(`${p}.sections.network`)}</SectionTitle>
      {si.localIps.length > 0 && (
        <InfoRow
          label={t(`${p}.localIps`)}
          value={
            <div className='flex flex-col items-end gap-0.5'>
              {si.localIps.map(ip => (
                <span key={ip} className='font-mono text-xs'>
                  {ip}
                </span>
              ))}
            </div>
          }
        />
      )}
      {si.macAddress && (
        <InfoRow
          label={t(`${p}.macAddress`)}
          value={<span className='font-mono text-xs'>{si.macAddress}</span>}
        />
      )}

      {/* Security */}
      <SectionTitle>{t(`${p}.sections.security`)}</SectionTitle>
      <InfoRow
        label={t(`${p}.antivirus`)}
        value={
          si.antivirus.length > 0 ? (
            <div className='flex flex-col items-end gap-0.5'>
              {si.antivirus.map(av => (
                <span key={av}>{av}</span>
              ))}
            </div>
          ) : (
            <span className='text-muted-foreground'>{t(`${p}.none`)}</span>
          )
        }
      />
      <InfoRow
        label={t(`${p}.firewall`)}
        value={
          si.firewallEnabled == null
            ? t(`${p}.unknown`)
            : si.firewallEnabled
              ? t(`${p}.enabled`)
              : t(`${p}.disabled`)
        }
      />

      {/* Agent */}
      <SectionTitle>{t(`${p}.sections.agent`)}</SectionTitle>
      <InfoRow label={t(`${p}.agentVersion`)} value={si.agentVersion} />
      {agent.lastHeartbeat && (
        <InfoRow
          label={t(`${p}.lastHeartbeat`)}
          value={formatDistanceToNow(new Date(agent.lastHeartbeat), {
            addSuffix: true,
            locale
          })}
        />
      )}
      {agent.lastConnectedAt && (
        <InfoRow
          label={t(`${p}.lastConnected`)}
          value={formatDistanceToNow(new Date(agent.lastConnectedAt), {
            addSuffix: true,
            locale
          })}
        />
      )}
      <InfoRow
        label={t(`${p}.createdAt`)}
        value={new Date(agent.createdAt).toLocaleDateString(i18n.language)}
      />
    </div>
  )
}
