import { UsersIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'
import type { Client } from '@/domain/types/audit.ts'

interface AuditClientsContentProps {
  clients: Client[]
}

export function AuditClientsContent({ clients }: AuditClientsContentProps) {
  const { t } = useTranslation('audit')

  if (clients.length === 0) {
    return (
      <Empty>
        <EmptyHeader>
          <EmptyMedia variant='icon'>
            <UsersIcon />
          </EmptyMedia>
          <EmptyTitle>{t('show.clients.empty')}</EmptyTitle>
        </EmptyHeader>
      </Empty>
    )
  }

  return (
    <div className='space-y-3'>
      {clients.map(client => (
        <div
          key={client.id}
          className='ring-foreground/10 bg-card flex items-center gap-4 rounded-2xl p-4 ring-1'
        >
          <img
            src={
              client.avatarUrl ||
              `https://ui-avatars.com/api/?name=${client.firstName}+${client.lastName}`
            }
            alt={client.username}
            className='size-10 rounded-full'
          />
          <div className='min-w-0'>
            <p className='text-sm font-medium'>
              {client.firstName} {client.lastName}
            </p>
            <p className='text-muted-foreground truncate text-sm'>
              {client.email}
            </p>
          </div>
        </div>
      ))}
    </div>
  )
}
