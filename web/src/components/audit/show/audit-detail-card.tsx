import { type Locale, format } from 'date-fns'
import { enUS, fr } from 'date-fns/locale'
import { Building2Icon } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import { Badge } from '@/components/ui/badge'
import { Card } from '@/components/ui/card'
import type { UserAudit } from '@/domain/types/audit.ts'

interface AuditDetailCardProps {
  audit: UserAudit
}

const DATE_LOCALES: Record<string, Locale> = { fr, en: enUS }

export function AuditDetailCard({ audit }: AuditDetailCardProps) {
  const { t, i18n } = useTranslation('audit')

  const locale = DATE_LOCALES[i18n.language] ?? enUS

  return (
    <Card className='p-6'>
      <div className='flex items-center gap-4'>
        <div className='bg-primary/10 text-primary flex size-12 items-center justify-center rounded-xl'>
          <Building2Icon className='size-6' />
        </div>

        <div className='flex-1 space-y-1'>
          <div className='flex items-center gap-x-2'>
            <h1 className='truncate text-lg font-semibold'>{audit.name}</h1>
            <Badge variant={audit.isArchived ? 'destructive' : 'secondary'}>
              {audit.isArchived
                ? t('show.status.archived')
                : t('show.status.active')}
            </Badge>
          </div>

          <p className='text-muted-foreground flex items-center gap-x-4 text-sm'>
            <span className='flex items-center gap-x-1.5'>
              <Building2Icon className='size-3.5' />
              {audit.company.name}
            </span>
            <span>
              {t('show.endDate')} :{' '}
              {format(new Date(audit.endDate), 'dd MMMM yyyy', { locale })}
            </span>
          </p>
        </div>
      </div>
    </Card>
  )
}
