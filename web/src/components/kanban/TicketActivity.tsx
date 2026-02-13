import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { Clock } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import type { TicketResponse } from '@/domain/types/Kanban'

type TicketActivityProps = {
  ticket: TicketResponse
}

const TicketActivity = ({ ticket }: TicketActivityProps) => {
  const { t } = useTranslation()

  return (
    <div className='space-y-4'>
      <h3 className='text-lg font-semibold text-gray-900 dark:text-gray-100'>
        {t('kanban:ticket.activity')}
      </h3>

      <div className='space-y-4'>
        <div className='flex gap-3'>
          <div className='flex h-8 w-8 items-center justify-center rounded-full bg-orange-100 dark:bg-orange-900/20'>
            <Clock className='h-4 w-4 text-orange-600 dark:text-orange-400' />
          </div>
          <div className='flex-1'>
            <div className='flex items-center gap-2'>
              <span className='font-medium text-gray-900 dark:text-gray-100'>
                {ticket.createdBy.firstName} {ticket.createdBy.lastName}
              </span>
              <span className='text-sm text-gray-500 dark:text-gray-400'>
                {t('kanban:ticket.createdTicket')}
              </span>
            </div>
            <div className='text-xs text-gray-500 dark:text-gray-400'>
              {format(new Date(ticket.createdAt), 'PPp', { locale: fr })}
            </div>
          </div>
        </div>

        {ticket.updatedAt !== ticket.createdAt && (
          <div className='flex gap-3'>
            <div className='flex h-8 w-8 items-center justify-center rounded-full bg-blue-100 dark:bg-blue-900/20'>
              <Clock className='h-4 w-4 text-blue-600 dark:text-blue-400' />
            </div>
            <div className='flex-1'>
              <div className='flex items-center gap-2'>
                <span className='text-sm text-gray-700 dark:text-gray-300'>
                  {t('kanban:ticket.updated')}
                </span>
              </div>
              <div className='text-xs text-gray-500 dark:text-gray-400'>
                {format(new Date(ticket.updatedAt), 'PPp', { locale: fr })}
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}

export default TicketActivity
