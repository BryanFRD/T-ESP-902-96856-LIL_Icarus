import { format } from 'date-fns'
import { fr } from 'date-fns/locale'
import { Archive, Building2, Calendar, ChevronRight } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'
import urlSlug from 'url-slug'

import {
  Card,
  CardContent,
  CardFooter,
  CardHeader
} from '@/components/ui/card/Card.tsx'
import type { UserAudit } from '@/domain/types/Audit.ts'
import { ROUTES, getRoutePath } from '@/router/routesName'

type AuditCardProps = {
  audit: UserAudit
}

const AuditCard = ({ audit }: AuditCardProps) => {
  const { t } = useTranslation()

  return (
    <Link
      to={`${getRoutePath(ROUTES.AUDITS)}/${urlSlug(audit.name)}`}
      state={{ audit }}
      className='group block'
    >
      <Card
        hover
        className='flex h-full flex-col justify-between overflow-hidden'
      >
        <CardHeader>
          <div className='mb-3 flex items-start justify-between'>
            <div className='flex h-10 w-10 items-center justify-center rounded-lg bg-orange-50 text-orange-600 dark:bg-orange-900/20 dark:text-orange-400'>
              {audit.isArchived ? (
                <Archive size={20} />
              ) : (
                <Building2 size={20} />
              )}
            </div>
            <span
              className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
                audit.isArchived
                  ? 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-300'
                  : 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300'
              }`}
            >
              {audit.isArchived
                ? t('audit:status.archived')
                : t('audit:status.active')}
            </span>
          </div>
          <h3 className='mb-1 truncate text-lg font-bold text-gray-900 dark:text-gray-100'>
            {audit.name}
          </h3>
          <p className='flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400'>
            <Building2 size={14} />
            <span className='truncate'>{audit.company.name}</span>
          </p>
        </CardHeader>

        <CardContent className='pb-4'>
          {audit.description && (
            <p className='line-clamp-2 text-sm text-gray-600 dark:text-gray-400'>
              {audit.description}
            </p>
          )}
        </CardContent>

        <CardFooter className='mt-auto border-t border-gray-100 pt-4 dark:border-gray-800'>
          <div className='flex w-full items-center justify-between'>
            <div className='flex items-center gap-4 text-xs text-gray-500'>
              <div
                className='flex items-center gap-1.5'
                title={t('audit:endDate')}
              >
                <Calendar size={14} />
                <span>
                  {format(new Date(audit.endDate), 'dd MMM yyyy', {
                    locale: fr
                  })}
                </span>
              </div>

              {audit.clients.length > 0 && (
                <div className='flex -space-x-2'>
                  {audit.clients.slice(0, 3).map(client => (
                    <img
                      key={client.id}
                      src={
                        client.avatarUrl ||
                        `https://ui-avatars.com/api/?name=${client.firstName}+${client.lastName}`
                      }
                      alt={client.username}
                      className='h-6 w-6 rounded-full border-2 border-white ring-1 ring-gray-100 dark:border-gray-900 dark:ring-gray-800'
                      title={`${client.firstName} ${client.lastName}`}
                    />
                  ))}
                  {audit.clients.length > 3 && (
                    <div className='flex h-6 w-6 items-center justify-center rounded-full border-2 border-white bg-gray-100 text-[10px] font-medium text-gray-600 dark:border-gray-900 dark:bg-gray-800 dark:text-gray-300'>
                      +{audit.clients.length - 3}
                    </div>
                  )}
                </div>
              )}
            </div>

            <div className='flex items-center text-orange-500 opacity-0 transition-opacity duration-300 group-hover:opacity-100'>
              <ChevronRight size={18} />
            </div>
          </div>
        </CardFooter>
      </Card>
    </Link>
  )
}

export default AuditCard
