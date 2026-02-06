import { Loader2, Plus } from 'lucide-react'
import { useTranslation } from 'react-i18next'

import AuditAccessModal from '@/components/audit/AuditAccessModal'
import AuditList from '@/components/audit/AuditList'
import Button from '@/components/ui/button/Button'
import { useCheckAuditsAccess } from '@/hooks/audit/useCheckAuditsAccess'
import ROUTES, { getRoutePath } from '@/router/routesName'

// Import de la liste

const AuditScreen = () => {
  const { t } = useTranslation()
  const { data: accessData, isLoading } = useCheckAuditsAccess()

  if (isLoading) {
    return (
      <div className='flex h-full w-full items-center justify-center p-8'>
        <Loader2 className='h-8 w-8 animate-spin text-orange-500' />
      </div>
    )
  }

  if (!accessData?.hasAccess) {
    return (
      <>
        <AuditAccessModal isOpen={true} />
        <div className='pointer-events-none relative h-full w-full p-8 opacity-50 blur-sm filter select-none'>
          <div className='mb-8 flex items-center justify-between'>
            <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
              {t('audit:title')}
            </h1>
          </div>
          <div className='grid gap-6 sm:grid-cols-2 lg:grid-cols-3'>
            {[...Array(6)].map((_, i) => (
              <div
                key={i}
                className='h-64 rounded-xl bg-gray-100 dark:bg-gray-800'
              ></div>
            ))}
          </div>
        </div>
      </>
    )
  }

  return (
    <div className='p-8'>
      <div className='mb-8 flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between'>
        <div>
          <h1 className='text-2xl font-bold text-gray-900 dark:text-white'>
            {t('audit:title')}
          </h1>
          <p className='mt-1 text-sm text-gray-500 dark:text-gray-400'>
            {t('audit:subtitle')}
          </p>
        </div>

        <Button
          as='link'
          to={getRoutePath(ROUTES.CREATE_AUDIT)}
          variant='primary'
          className='shrink-0'
        >
          <Plus className='mr-2 h-4 w-4' />
          {t('audit:createAudit')}
        </Button>
      </div>

      <AuditList />
    </div>
  )
}

export default AuditScreen
