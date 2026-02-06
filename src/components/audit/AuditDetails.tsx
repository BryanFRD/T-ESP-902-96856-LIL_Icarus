import { useTranslation } from 'react-i18next'
import { useLocation } from 'react-router-dom'

import type { UserAudit } from '@/domain/types/Audit.ts'

const AuditDetails = () => {
  const { t } = useTranslation()
  const location = useLocation()
  const audit = location.state.audit as UserAudit

  if (!audit) {
    return <div>{t('audit:details.notFound')}</div>
  }

  return (
    <div>
      <h1 className='mb-4 text-2xl font-bold'>{audit.name}</h1>
      <p className='mb-2'>
        <strong>{t('audit:details.company')}:</strong> {audit.company.name}
      </p>
      <p className='mb-2'>
        <strong>{t('audit:details.description')}:</strong>{' '}
        {audit.description || t('audit:details.noDescription')}
      </p>
      <p className='mb-2'>
        <strong>{t('audit:details.endDate')}:</strong>{' '}
        {new Date(audit.endDate).toLocaleDateString()}
      </p>
      <p className='mb-2'>
        <strong>{t('audit:details.status')}:</strong>{' '}
        {audit.isArchived
          ? t('audit:status.archived')
          : new Date(audit.endDate) < new Date()
            ? t('audit:status.expired')
            : t('audit:status.active')}
      </p>
    </div>
  )
}

export default AuditDetails
