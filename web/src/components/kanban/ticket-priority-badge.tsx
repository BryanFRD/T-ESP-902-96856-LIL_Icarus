import { ChevronDown, ChevronUp, ChevronsUp, Equal } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { twMerge } from 'tailwind-merge'

import type { TicketPriority } from '@/domain/types/kanban.ts'

type TicketPriorityBadgeProps = {
  priority: TicketPriority
  className?: string
  size?: 'sm' | 'md'
  variant?: 'badge' | 'icon'
}

const TicketPriorityBadge = ({
  priority,
  className,
  size = 'md',
  variant = 'badge'
}: TicketPriorityBadgeProps) => {
  const { t } = useTranslation()

  const getPriorityConfig = () => {
    switch (priority) {
      case 'CRITICAL':
        return {
          label: t('kanban:priority.critical'),
          icon: ChevronsUp,
          colorClass: 'text-[#CD1317] dark:text-red-400',
          bgClass: 'bg-red-50 dark:bg-red-900/10'
        }
      case 'HIGH':
        return {
          label: t('kanban:priority.high'),
          icon: ChevronUp,
          colorClass: 'text-[#CD1317] dark:text-red-400',
          bgClass: 'bg-red-50 dark:bg-red-900/10'
        }
      case 'MEDIUM':
        return {
          label: t('kanban:priority.medium'),
          icon: Equal,
          colorClass: 'text-[#FF8B00] dark:text-orange-400',
          bgClass: 'bg-orange-50 dark:bg-orange-900/10'
        }
      case 'LOW':
        return {
          label: t('kanban:priority.low'),
          icon: ChevronDown,
          colorClass: 'text-[#0065FF] dark:text-blue-400',
          bgClass: 'bg-blue-50 dark:bg-blue-900/10'
        }
    }
  }

  const config = getPriorityConfig()
  const Icon = config.icon

  if (variant === 'icon') {
    return (
      <div
        className={twMerge(
          'flex items-center justify-center',
          config.colorClass,
          className
        )}
        title={config.label}
      >
        <Icon className='h-4 w-4' strokeWidth={2.5} />
      </div>
    )
  }

  const sizeClasses =
    size === 'sm' ? 'px-1.5 py-0.5 text-[11px]' : 'px-2 py-1 text-xs'

  return (
    <span
      className={twMerge(
        'inline-flex items-center gap-1.5 rounded-[3px] font-bold tracking-wider uppercase',
        config.bgClass,
        config.colorClass,
        sizeClasses,
        className
      )}
    >
      <Icon className='h-3.5 w-3.5' strokeWidth={2.5} />
      {config.label}
    </span>
  )
}

export default TicketPriorityBadge
