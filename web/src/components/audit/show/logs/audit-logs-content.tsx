import {
  Activity,
  AlertCircle,
  ChevronLeft,
  ChevronRight,
  Filter,
  Loader2,
  LogIn,
  LogOut,
  Monitor,
  MonitorOff,
  Pencil,
  Plus,
  Shield,
  Terminal,
  Trash2,
  User2
} from 'lucide-react'
import { useState } from 'react'
import { useTranslation } from 'react-i18next'

import { Avatar, AvatarFallback } from '@/components/ui/avatar'
import { Badge } from '@/components/ui/badge'
import { Button } from '@/components/ui/button'
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue
} from '@/components/ui/select'
import type { AuditLogEntry } from '@/domain/types/audit.ts'
import { useAuditLogs } from '@/hooks/audit/use-audit-logs.ts'

interface AuditLogsContentProps {
  auditId: string
}

const ACTION_FILTERS = [
  { value: 'all', label: 'All Actions' },
  { value: 'Create', label: 'Create' },
  { value: 'Read', label: 'Read' },
  { value: 'Update', label: 'Update' },
  { value: 'Delete', label: 'Delete' },
  { value: 'AgentConnect', label: 'Agent Connect' },
  { value: 'AgentDisconnect', label: 'Agent Disconnect' },
  { value: 'AgentCommand', label: 'Agent Command' },
  { value: 'AgentShutdown', label: 'Agent Shutdown' },
  { value: 'AgentDownload', label: 'Agent Download' },
  { value: 'Login', label: 'Login' },
  { value: 'Logout', label: 'Logout' }
]

const PAGE_SIZE = 25

function getActionIcon(action: string) {
  switch (action) {
    case 'Create':
    case 'InvitationCreate':
      return Plus
    case 'Delete':
    case 'InvitationCancel':
      return Trash2
    case 'Update':
    case 'AvatarUpload':
      return Pencil
    case 'Login':
      return LogIn
    case 'Logout':
      return LogOut
    case 'AgentConnect':
      return Monitor
    case 'AgentDisconnect':
      return MonitorOff
    case 'AgentCommand':
      return Terminal
    case 'AgentShutdown':
      return Shield
    default:
      return Activity
  }
}

function getActionBadgeClasses(action: string): string {
  switch (action) {
    case 'Create':
    case 'InvitationCreate':
    case 'AgentConnect':
    case 'Register':
      return 'border-emerald-500/30 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400'
    case 'Delete':
    case 'InvitationCancel':
    case 'AgentShutdown':
      return 'border-red-500/30 bg-red-500/10 text-red-600 dark:text-red-400'
    case 'Update':
    case 'AvatarUpload':
    case 'AvatarDelete':
      return 'border-amber-500/30 bg-amber-500/10 text-amber-600 dark:text-amber-400'
    case 'Login':
    case 'Logout':
    case 'PasswordReset':
    case 'EmailConfirm':
      return 'border-blue-500/30 bg-blue-500/10 text-blue-600 dark:text-blue-400'
    case 'AgentCommand':
      return 'border-violet-500/30 bg-violet-500/10 text-violet-600 dark:text-violet-400'
    case 'AgentDisconnect':
      return 'border-orange-500/30 bg-orange-500/10 text-orange-600 dark:text-orange-400'
    case 'Read':
      return 'border-zinc-500/30 bg-zinc-500/10 text-zinc-600 dark:text-zinc-400'
    default:
      return 'border-zinc-500/30 bg-zinc-500/10 text-zinc-500'
  }
}

function getResourceBadgeClasses(resourceType: string): string {
  switch (resourceType) {
    case 'Agent':
      return 'border-violet-500/20 bg-violet-500/5 text-violet-600 dark:text-violet-400'
    case 'Ticket':
    case 'TicketComment':
    case 'TicketAttachment':
    case 'TicketLabel':
    case 'KanbanColumn':
      return 'border-sky-500/20 bg-sky-500/5 text-sky-600 dark:text-sky-400'
    case 'Audit':
    case 'AuditClient':
    case 'AuditInvitation':
      return 'border-amber-500/20 bg-amber-500/5 text-amber-600 dark:text-amber-400'
    case 'Auth':
      return 'border-blue-500/20 bg-blue-500/5 text-blue-600 dark:text-blue-400'
    default:
      return 'border-zinc-500/20 bg-zinc-500/5 text-zinc-500'
  }
}

function getMethodColor(method: string): string {
  switch (method) {
    case 'GET':
      return 'text-emerald-600 dark:text-emerald-400'
    case 'POST':
      return 'text-blue-600 dark:text-blue-400'
    case 'PUT':
    case 'PATCH':
      return 'text-amber-600 dark:text-amber-400'
    case 'DELETE':
      return 'text-red-600 dark:text-red-400'
    default:
      return 'text-muted-foreground'
  }
}

function formatAction(action: string) {
  return action.replace(/_/g, ' ').replace(/([a-z])([A-Z])/g, '$1 $2')
}

function formatResourceType(type: string) {
  return type.replace(/([a-z])([A-Z])/g, '$1 $2')
}

function formatTimestamp(timestamp: string) {
  const raw = timestamp.endsWith('Z') ? timestamp : `${timestamp}Z`
  const date = new Date(raw)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMin = Math.floor(diffMs / 60_000)
  const diffHours = Math.floor(diffMs / 3_600_000)

  if (diffMin < 1) return 'Just now'
  if (diffMin < 60) return `${String(diffMin)}m ago`
  if (diffHours < 24) return `${String(diffHours)}h ago`

  return date.toLocaleDateString(undefined, {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

function LogEntry({ entry }: { entry: AuditLogEntry }) {
  const Icon = getActionIcon(entry.action ?? '')
  const actionClasses = getActionBadgeClasses(entry.action ?? '')
  const resourceClasses = getResourceBadgeClasses(entry.resourceType ?? '')

  return (
    <div className='border-border/50 hover:bg-muted/30 flex items-start gap-3 border-b px-2 py-3 transition-colors last:border-b-0'>
      <div
        className={`mt-0.5 ${actionClasses.split(' ').find(c => c.startsWith('text-')) ?? 'text-muted-foreground'}`}
      >
        <Icon className='h-4 w-4' />
      </div>

      <div className='min-w-0 flex-1 space-y-1'>
        <div className='flex flex-wrap items-center gap-1.5'>
          <Badge
            variant='outline'
            className={`px-1.5 py-0 text-[11px] font-medium ${actionClasses}`}
          >
            {formatAction(entry.action)}
          </Badge>
          <Badge
            variant='outline'
            className={`px-1.5 py-0 text-[11px] ${resourceClasses}`}
          >
            {formatResourceType(entry.resourceType)}
          </Badge>
          <span
            className={`font-mono text-[11px] font-semibold ${getMethodColor(entry.requestMethod ?? '')}`}
          >
            {entry.requestMethod ?? ''}
          </span>
        </div>

        <div className='text-muted-foreground flex items-center gap-2 text-xs'>
          {entry.user ? (
            <div className='flex items-center gap-1.5'>
              <Avatar className='h-4 w-4'>
                <AvatarFallback className='bg-primary/10 text-primary text-[8px]'>
                  {(entry.user.firstName ?? '?')[0]}
                  {(entry.user.lastName ?? '?')[0]}
                </AvatarFallback>
              </Avatar>
              <span className='text-foreground/80 font-medium'>
                {entry.user.firstName ?? ''} {entry.user.lastName ?? ''}
              </span>
            </div>
          ) : (
            <div className='flex items-center gap-1'>
              <User2 className='h-3 w-3' />
              <span className='italic'>System</span>
            </div>
          )}
          <span className='text-muted-foreground/60'>·</span>
          <span>{formatTimestamp(entry.createdAt)}</span>
          <span className='hidden truncate font-mono text-[10px] opacity-40 sm:inline'>
            {entry.requestPath}
          </span>
        </div>

        {entry.metadata &&
          typeof entry.metadata === 'object' &&
          Object.keys(entry.metadata).length > 0 && (
            <div className='flex flex-wrap gap-1.5'>
              {'response_status' in entry.metadata && (
                <span
                  className={`rounded px-1 py-0.5 font-mono text-[10px] ${
                    Number(entry.metadata.response_status) >= 400
                      ? 'bg-red-500/10 text-red-500'
                      : 'bg-emerald-500/10 text-emerald-500'
                  }`}
                >
                  {String(entry.metadata.response_status)}
                </span>
              )}
              {'duration_ms' in entry.metadata && (
                <span className='text-muted-foreground/60 font-mono text-[10px]'>
                  {String(entry.metadata.duration_ms)}ms
                </span>
              )}
              {'agent_id' in entry.metadata && (
                <span className='rounded bg-violet-500/10 px-1 py-0.5 font-mono text-[10px] text-violet-500'>
                  agent:{String(entry.metadata.agent_id).slice(0, 8)}
                </span>
              )}
              {'agent_name' in entry.metadata && (
                <span className='rounded bg-violet-500/10 px-1 py-0.5 text-[10px] text-violet-500'>
                  {String(entry.metadata.agent_name)}
                </span>
              )}
            </div>
          )}
      </div>
    </div>
  )
}

export function AuditLogsContent({ auditId }: AuditLogsContentProps) {
  const { t } = useTranslation('audit')
  const [actionFilter, setActionFilter] = useState('all')
  const [page, setPage] = useState(0)

  const offset = page * PAGE_SIZE
  const { data, isLoading, isError } = useAuditLogs(
    auditId,
    PAGE_SIZE,
    offset,
    actionFilter === 'all' ? undefined : actionFilter
  )

  if (isLoading) {
    return (
      <div className='flex items-center justify-center py-12'>
        <Loader2 className='text-muted-foreground h-6 w-6 animate-spin' />
      </div>
    )
  }

  if (isError) {
    return (
      <div className='text-muted-foreground flex flex-col items-center justify-center gap-2 py-12'>
        <AlertCircle className='h-6 w-6' />
        <p className='text-sm'>{t('show.logs.error')}</p>
      </div>
    )
  }

  const totalPages = Math.ceil((data?.total ?? 0) / PAGE_SIZE)

  return (
    <div className='space-y-3'>
      <div className='flex items-center justify-between'>
        <div className='flex items-center gap-2'>
          <Filter className='text-muted-foreground h-4 w-4' />
          <Select
            value={actionFilter}
            onValueChange={v => {
              setActionFilter(v)
              setPage(0)
            }}
          >
            <SelectTrigger className='h-8 w-[180px] text-xs'>
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {ACTION_FILTERS.map(f => (
                <SelectItem key={f.value} value={f.value}>
                  {f.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        <span className='text-muted-foreground text-xs'>
          {t('show.logs.total', { count: data?.total ?? 0 })}
        </span>
      </div>

      {!data?.data.length ? (
        <div className='bg-card text-muted-foreground flex flex-col items-center justify-center gap-2 rounded-lg border py-12'>
          <Activity className='h-6 w-6' />
          <p className='text-sm'>{t('show.logs.empty')}</p>
        </div>
      ) : (
        <>
          <div className='bg-card rounded-lg border'>
            {data.data.map(entry => (
              <LogEntry key={entry.id} entry={entry} />
            ))}
          </div>

          {totalPages > 1 && (
            <div className='flex items-center justify-center gap-2'>
              <Button
                variant='outline'
                size='sm'
                disabled={page === 0}
                onClick={() => setPage(p => p - 1)}
              >
                <ChevronLeft className='h-4 w-4' />
              </Button>
              <span className='text-muted-foreground text-xs'>
                {page + 1} / {totalPages}
              </span>
              <Button
                variant='outline'
                size='sm'
                disabled={page >= totalPages - 1}
                onClick={() => setPage(p => p + 1)}
              >
                <ChevronRight className='h-4 w-4' />
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  )
}
