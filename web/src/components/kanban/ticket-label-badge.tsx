import { X } from 'lucide-react'
import { twMerge } from 'tailwind-merge'

import type { LabelResponse } from '@/domain/types/kanban.ts'

type TicketLabelBadgeProps = {
  label: LabelResponse
  onRemove?: () => void
  className?: string
  size?: 'sm' | 'md'
}

const TicketLabelBadge = ({
  label,
  onRemove,
  className,
  size = 'md'
}: TicketLabelBadgeProps) => {
  const getTextColor = (hexColor: string) => {
    const r = Number.parseInt(hexColor.slice(1, 3), 16)
    const g = Number.parseInt(hexColor.slice(3, 5), 16)
    const b = Number.parseInt(hexColor.slice(5, 7), 16)
    const brightness = (r * 299 + g * 587 + b * 114) / 1000
    return brightness > 160 ? 'text-[#172B4D]' : 'text-white'
  }

  const sizeClasses = {
    xs: 'px-1.5 py-[1px] text-[10px] font-bold uppercase',
    sm: 'px-2 py-0.5 text-xs font-semibold',
    md: 'px-2.5 py-1 text-sm font-medium'
  }

  return (
    <span
      className={twMerge(
        'inline-flex max-w-[120px] items-center gap-1 truncate rounded-[3px]',
        getTextColor(label.color),
        sizeClasses[size],
        className
      )}
      style={{ backgroundColor: label.color }}
      title={label.name}
    >
      <span className='truncate'>{label.name}</span>
      {onRemove && (
        <button
          type='button'
          onClick={e => {
            e.stopPropagation()
            onRemove()
          }}
          className='ml-0.5 rounded-sm hover:opacity-70'
        >
          <X className='h-3 w-3' />
        </button>
      )}
    </span>
  )
}

export default TicketLabelBadge
