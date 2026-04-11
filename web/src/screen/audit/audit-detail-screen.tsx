import { ShieldIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { useLocation } from 'react-router-dom'

import { AuditDetailCard } from '@/components/audit/show/audit-detail-card'
import { AuditTabs } from '@/components/audit/show/audit-tabs'
import {
  Empty,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'
import type { UserAudit } from '@/domain/types/audit.ts'

export default function AuditDetails() {
  const { t } = useTranslation('audit')

  const location = useLocation()
  const audit = location.state?.audit as UserAudit

  if (!audit) {
    return (
      <Empty>
        <EmptyHeader>
          <EmptyMedia variant='icon'>
            <ShieldIcon />
          </EmptyMedia>
          <EmptyTitle>{t('show.notFound')}</EmptyTitle>
          <EmptyDescription>{t('show.notFoundDescription')}</EmptyDescription>
        </EmptyHeader>
      </Empty>
    )
  }

  return (
    <div className='mx-auto w-full max-w-5xl space-y-6 px-4 py-8'>
      <AuditDetailCard audit={audit} />

      <AuditTabs audit={audit} />
    </div>
  )
}
