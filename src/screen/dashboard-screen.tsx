import {
  AlertTriangle,
  FileText,
  MessageSquare,
  Monitor,
  Shield,
  Ticket,
  TrendingUp,
  Users
} from 'lucide-react'
import { useTranslation } from 'react-i18next'

import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Skeleton } from '@/components/ui/skeleton'
import type {
  ActivityMetrics,
  UserDashboardMetrics
} from '@/domain/types/dashboard.ts'
import { useDashboard } from '@/hooks/dashboard/use-dashboard.ts'

function StatCard({
  title,
  value,
  icon: Icon,
  detail,
  iconColor
}: {
  title: string
  value: number | string
  icon: React.ElementType
  detail?: string
  iconColor?: string
}) {
  return (
    <Card>
      <CardHeader className='flex flex-row items-center justify-between pb-2'>
        <CardTitle className='text-muted-foreground text-sm font-medium'>
          {title}
        </CardTitle>
        <Icon className={`h-4 w-4 ${iconColor ?? 'text-muted-foreground'}`} />
      </CardHeader>
      <CardContent>
        <div className='text-2xl font-bold'>{value}</div>
        {detail && (
          <p className='text-muted-foreground mt-1 text-xs'>{detail}</p>
        )}
      </CardContent>
    </Card>
  )
}

function ActivityCard({
  title,
  icon: Icon,
  metrics,
  iconColor
}: {
  title: string
  icon: React.ElementType
  metrics: ActivityMetrics
  iconColor?: string
}) {
  const { t } = useTranslation('app')

  return (
    <Card>
      <CardHeader className='flex flex-row items-center justify-between pb-2'>
        <CardTitle className='text-muted-foreground text-sm font-medium'>
          {title}
        </CardTitle>
        <Icon className={`h-4 w-4 ${iconColor ?? 'text-muted-foreground'}`} />
      </CardHeader>
      <CardContent className='space-y-2'>
        <div className='text-2xl font-bold'>{metrics.total}</div>
        <div className='text-muted-foreground flex gap-4 text-xs'>
          <span>
            {t('dashboard.last7Days')}: {metrics.last7Days}
          </span>
          <span>
            {t('dashboard.last30Days')}: {metrics.last30Days}
          </span>
        </div>
      </CardContent>
    </Card>
  )
}

function PriorityBar({
  label,
  assigned,
  total,
  maxTotal,
  color,
  colorDark
}: {
  label: string
  assigned: number
  total: number
  maxTotal: number
  color: string
  colorDark: string
}) {
  const assignedPct = maxTotal > 0 ? (assigned / maxTotal) * 100 : 0
  const unassignedPct = maxTotal > 0 ? ((total - assigned) / maxTotal) * 100 : 0

  return (
    <div className='flex items-center gap-2'>
      <span className='text-muted-foreground w-16 text-xs'>{label}</span>
      <div className='bg-muted relative h-2 flex-1 overflow-hidden rounded-full'>
        <div
          className={`absolute left-0 h-2 ${color}`}
          style={{ width: `${String(assignedPct + unassignedPct)}%` }}
        />
        <div
          className={`absolute left-0 h-2 ${colorDark}`}
          style={{ width: `${String(assignedPct)}%` }}
        />
      </div>
      <span className='w-10 text-right text-xs font-medium'>
        {assigned}/{total}
      </span>
    </div>
  )
}

function TicketsCard({ metrics }: { metrics: UserDashboardMetrics }) {
  const { t } = useTranslation('app')
  const total =
    metrics.tickets.byPriority.critical +
    metrics.tickets.byPriority.high +
    metrics.tickets.byPriority.medium +
    metrics.tickets.byPriority.low

  return (
    <Card className='col-span-1 md:col-span-2'>
      <CardHeader className='flex flex-row items-center justify-between pb-2'>
        <CardTitle className='text-muted-foreground text-sm font-medium'>
          {t('dashboard.tickets.title')}
        </CardTitle>
        <Ticket className='h-4 w-4 text-sky-500' />
      </CardHeader>
      <CardContent>
        <div className='grid grid-cols-1 gap-6 sm:grid-cols-2'>
          <div className='space-y-3'>
            <div className='flex items-baseline gap-2'>
              <span className='text-2xl font-bold'>
                {metrics.tickets.assigned}
              </span>
              <span className='text-muted-foreground text-xs'>
                {t('dashboard.tickets.assigned')}
              </span>
            </div>
            <div className='text-muted-foreground flex gap-4 text-xs'>
              <span className='flex items-center gap-1'>
                <FileText className='h-3 w-3' />
                {metrics.tickets.created} {t('dashboard.tickets.created')}
              </span>
              {metrics.tickets.overdue > 0 && (
                <span className='flex items-center gap-1 text-red-500'>
                  <AlertTriangle className='h-3 w-3' />
                  {metrics.tickets.overdue} {t('dashboard.tickets.overdue')}
                </span>
              )}
            </div>
          </div>

          <div className='space-y-2'>
            <p className='text-muted-foreground text-xs font-medium'>
              {t('dashboard.tickets.byPriority')}
            </p>
            <PriorityBar
              label={t('dashboard.priority.critical')}
              assigned={metrics.tickets.byPriorityAssigned.critical}
              total={metrics.tickets.byPriority.critical}
              maxTotal={total}
              color='bg-red-500/30'
              colorDark='bg-red-500'
            />
            <PriorityBar
              label={t('dashboard.priority.high')}
              assigned={metrics.tickets.byPriorityAssigned.high}
              total={metrics.tickets.byPriority.high}
              maxTotal={total}
              color='bg-orange-500/30'
              colorDark='bg-orange-500'
            />
            <PriorityBar
              label={t('dashboard.priority.medium')}
              assigned={metrics.tickets.byPriorityAssigned.medium}
              total={metrics.tickets.byPriority.medium}
              maxTotal={total}
              color='bg-yellow-500/30'
              colorDark='bg-yellow-500'
            />
            <PriorityBar
              label={t('dashboard.priority.low')}
              assigned={metrics.tickets.byPriorityAssigned.low}
              total={metrics.tickets.byPriority.low}
              maxTotal={total}
              color='bg-emerald-500/30'
              colorDark='bg-emerald-500'
            />
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

function DashboardContent({ data }: { data: UserDashboardMetrics }) {
  const { t } = useTranslation('app')

  return (
    <div className='space-y-6'>
      <div className='grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4'>
        <StatCard
          title={t('dashboard.teams.title')}
          value={data.teams.total}
          icon={Users}
          iconColor='text-blue-500'
          detail={`${String(data.teams.asOwner)} ${t('dashboard.teams.owner')} · ${String(data.teams.asMember)} ${t('dashboard.teams.member')}`}
        />
        <StatCard
          title={t('dashboard.audits.title')}
          value={data.audits.active}
          icon={Shield}
          iconColor='text-primary'
          detail={`${String(data.audits.asTeamMember)} ${t('dashboard.audits.asTeam')} · ${String(data.audits.archived)} ${t('dashboard.audits.archived')}`}
        />
        <StatCard
          title={t('dashboard.tickets.title')}
          value={data.tickets.assigned}
          icon={Ticket}
          iconColor='text-sky-500'
          detail={
            data.tickets.overdue > 0
              ? `${String(data.tickets.overdue)} ${t('dashboard.tickets.overdue')}`
              : t('dashboard.tickets.noneOverdue')
          }
        />
        <StatCard
          title={t('dashboard.access.title')}
          value={data.audits.asTeamMember + data.audits.asClient}
          icon={Monitor}
          iconColor='text-emerald-500'
          detail={`${String(data.audits.active)} ${t('dashboard.audits.active')}`}
        />
      </div>

      <div className='grid grid-cols-1 gap-4 md:grid-cols-2'>
        <TicketsCard metrics={data} />
      </div>

      <div className='grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3'>
        <ActivityCard
          title={t('dashboard.messages.title')}
          icon={MessageSquare}
          metrics={data.messages}
          iconColor='text-violet-500'
        />
        <ActivityCard
          title={t('dashboard.comments.title')}
          icon={FileText}
          metrics={data.comments}
          iconColor='text-amber-500'
        />
        <ActivityCard
          title={t('dashboard.events.title')}
          icon={TrendingUp}
          metrics={data.events}
          iconColor='text-emerald-500'
        />
      </div>
    </div>
  )
}

function DashboardSkeleton() {
  return (
    <div className='space-y-6'>
      <div className='grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4'>
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} className='h-28 rounded-2xl' />
        ))}
      </div>
      <Skeleton className='h-40 rounded-2xl' />
      <div className='grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3'>
        {Array.from({ length: 3 }).map((_, i) => (
          <Skeleton key={i} className='h-28 rounded-2xl' />
        ))}
      </div>
    </div>
  )
}

const DashboardScreen = () => {
  const { t } = useTranslation('app')
  const { data, isLoading } = useDashboard()

  return (
    <div className='mx-auto w-full max-w-6xl space-y-6 px-4 py-8'>
      <div className='space-y-1'>
        <h1 className='text-2xl font-semibold'>{t('dashboard.title')}</h1>
        <p className='text-muted-foreground text-sm'>
          {t('dashboard.subtitle')}
        </p>
      </div>

      {isLoading ? (
        <DashboardSkeleton />
      ) : data ? (
        <DashboardContent data={data} />
      ) : null}
    </div>
  )
}

export default DashboardScreen
