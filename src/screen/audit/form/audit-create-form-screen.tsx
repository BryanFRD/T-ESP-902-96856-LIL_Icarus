import { ArrowLeftIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { AuditCreateForm } from '@/components/audit/create/audit-create-form'
import { Button } from '@/components/ui/button'
import ROUTES, { getRoutePath } from '@/router/routes-name.ts'

const AuditCreateFormScreen = () => {
  const { t } = useTranslation('audit')

  return (
    <div className='mx-auto max-w-5xl space-y-8 px-4 py-8'>
      <div className='space-y-1'>
        <Button asChild variant='ghost' size='sm'>
          <Link to={getRoutePath(ROUTES.AUDITS)}>
            <ArrowLeftIcon className='size-4' />
            {t('show.backToAudits')}
          </Link>
        </Button>

        <h1 className='text-2xl font-semibold'>{t('create.title')}</h1>
      </div>

      <AuditCreateForm />
    </div>
  )
}

export default AuditCreateFormScreen
