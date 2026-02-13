import { Plus, SearchX } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import AuditCard from '@/components/audit/AuditCard'
import Button from '@/components/ui/button/Button'
import { useUserAudits } from '@/hooks/audit/useUserAudits'
import ROUTES, { getRoutePath } from '@/router/routesName'

const AuditList = () => {
  const { t } = useTranslation()
  const { data: audits, isLoading, isError } = useUserAudits()

  if (isLoading) {
    return (
      <div className='grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
        {[...Array(4)].map((_, i) => (
          <div
            key={i}
            className='h-64 animate-pulse rounded-xl bg-gray-100 dark:bg-gray-800'
          />
        ))}
      </div>
    )
  }

  if (isError) {
    return (
      <div className='flex h-64 flex-col items-center justify-center rounded-xl border border-dashed border-red-200 bg-red-50 text-center dark:border-red-900/50 dark:bg-red-900/20'>
        <p className='text-red-600 dark:text-red-400'>
          {t('audit:list.error')}
        </p>
      </div>
    )
  }

  if (!audits || audits.length === 0) {
    return (
      <div className='flex h-96 flex-col items-center justify-center rounded-xl border border-dashed border-gray-300 bg-gray-50/50 text-center dark:border-gray-700 dark:bg-gray-900/50'>
        <div className='mb-4 rounded-full bg-white p-4 shadow-sm dark:bg-gray-800'>
          <SearchX className='h-8 w-8 text-gray-400' />
        </div>
        <h3 className='text-lg font-semibold text-gray-900 dark:text-gray-100'>
          {t('audit:list.emptyTitle', 'Aucun audit trouvé')}
        </h3>
        <p className='mb-6 max-w-sm text-sm text-gray-500'>
          {t('audit:list.emptyDescription')}
        </p>
        <Button
          as='link'
          to={getRoutePath(ROUTES.CREATE_AUDIT)}
          variant='primary'
        >
          <Plus className='mr-2 h-4 w-4' />
          {t('audit:createAudit')}
        </Button>
      </div>
    )
  }

  return (
    <div className='grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
      {audits.map(audit => (
        <AuditCard key={audit.id} audit={audit} />
      ))}
    </div>
  )
}

export default AuditList
