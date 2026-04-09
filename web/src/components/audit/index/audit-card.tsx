import { type Locale, format } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import {
  ArchiveIcon,
  Building2Icon,
  CalendarIcon,
  ChevronRightIcon
} from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'
import urlSlug from 'url-slug'

import { Badge } from '@/components/ui/badge'
import {
  Card,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle
} from '@/components/ui/card'
import type { UserAudit } from '@/domain/types/audit.ts'
import { ROUTES, getRoutePath } from '@/router/routes-name.ts'

interface AuditCardProps {
  audit: UserAudit
}

const DATE_LOCALES: Record<string, Locale> = { fr, en: enUS }

export function AuditCard({ audit }: AuditCardProps) {
  const { t, i18n } = useTranslation('audit')

  const locale = DATE_LOCALES[i18n.language] ?? enUS

  return (
    <Link
      to={`${getRoutePath(ROUTES.AUDITS)}/${urlSlug(audit.name)}`}
      state={{ audit }}
      className='group'
    >
      <Card className='transition-shadow hover:shadow-md'>
        <CardHeader>
          <div className='flex items-start justify-between'>
            <div className='bg-primary/10 text-primary flex size-10 items-center justify-center rounded-xl'>
              {audit.isArchived ? (
                <ArchiveIcon className='size-5' />
              ) : (
                <Building2Icon className='size-5' />
              )}
            </div>

            <Badge variant={audit.isArchived ? 'destructive' : 'secondary'}>
              {audit.isArchived
                ? t('show.status.archived')
                : t('show.status.active')}
            </Badge>
          </div>

          <CardTitle className='truncate'>{audit.name}</CardTitle>
          <CardDescription className='flex items-center gap-x-1.5 truncate'>
            <Building2Icon className='size-3.5' />
            <span className='truncate'>{audit.company.name}</span>
          </CardDescription>
        </CardHeader>

        <CardFooter className='text-muted-foreground border-t text-xs'>
          <div className='flex w-full items-center justify-between'>
            <div className='flex items-center gap-4'>
              <div
                className='flex items-center gap-1.5'
                title={t('show.endDate')}
              >
                <CalendarIcon className='size-3.5' />
                <span>
                  {format(new Date(audit.endDate), 'dd MMM yyyy', { locale })}
                </span>
              </div>

              {/* {audit.clients.length > 0 && (
                <div className='flex -space-x-2'>
                  {audit.clients.slice(0, 3).map(client => (
                    <img
                      key={client.id}
                      src={
                        client.avatarUrl
                          ? buildAvatarUrl(client.avatarUrl)
                          : `https://ui-avatars.com/api/?name=${client.firstName}+${client.lastName}`
                      }
                      alt={client.username}
                      className='ring-background size-6 rounded-full ring-2'
                      title={`${client.firstName} ${client.lastName}`}
                    />
                  ))}
                  {audit.clients.length > 3 && (
                    <div className='bg-muted text-muted-foreground ring-background flex size-6 items-center justify-center rounded-full text-[10px] font-medium ring-2'>
                      +{audit.clients.length - 3}
                    </div>
                  )}
                </div>
              )} */}
            </div>

            <ChevronRightIcon className='size-4 opacity-0 transition-opacity group-hover:opacity-100' />
          </div>
        </CardFooter>
      </Card>
    </Link>
  )
}
