import { PlusIcon, SearchXIcon } from 'lucide-react'
import { useTranslation } from 'react-i18next'
import { Link } from 'react-router-dom'

import { AuditAccessDialog } from '@/components/audit/audit-access-dialog'
import { AuditCard } from '@/components/audit/index/audit-card'
import { Button } from '@/components/ui/button'
import {
  Empty,
  EmptyContent,
  EmptyDescription,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle
} from '@/components/ui/empty'
import { Skeleton } from '@/components/ui/skeleton'
import { useCheckAuditAccess } from '@/hooks/audit/use-check-audit-access.ts'
import { useUserAudit } from '@/hooks/audit/use-user-audit.ts'
import ROUTES, { getRoutePath } from '@/router/routes-name.ts'

const AuditScreen = () => {
  const { t } = useTranslation('audit')

  const { data: accessData, isLoading: isCheckingAccess } =
    useCheckAuditAccess()
  const { data: audits, isLoading: isLoadingAudits } = useUserAudit()

  if (isCheckingAccess || (isLoadingAudits && accessData?.hasAccess)) {
    return (
      <div className='grid grid-cols-1 gap-4 px-4 py-8 md:grid-cols-2 lg:grid-cols-3'>
        {Array.from({ length: 4 }).map((_, i) => (
          <Skeleton key={i} className='h-52 w-full rounded-2xl' />
        ))}
      </div>
    )
  }

  if (!accessData?.hasAccess) {
    return (
      <>
        <AuditAccessDialog open={true} />

        <div className='grid grid-cols-1 gap-4 px-4 py-8 md:grid-cols-2 lg:grid-cols-3'>
          {Array.from({ length: 4 }).map((_, i) => (
            <Skeleton key={i} className='h-52 w-full rounded-2xl' />
          ))}
        </div>
      </>
    )
  }

  if (!audits || audits.length === 0) {
    return (
      <Empty>
        <EmptyHeader>
          <EmptyMedia variant='icon'>
            <SearchXIcon />
          </EmptyMedia>
          <EmptyTitle>{t('index.empty.title')}</EmptyTitle>
          <EmptyDescription>{t('index.empty.description')}</EmptyDescription>
        </EmptyHeader>
        <EmptyContent>
          <Button asChild>
            <Link to={getRoutePath(ROUTES.CREATE_AUDIT)}>
              <PlusIcon className='size-4' />
              {t('index.createAudit')}
            </Link>
          </Button>
        </EmptyContent>
      </Empty>
    )
  }

  return (
    <div className='space-y-8 px-4 py-8'>
      <div className='flex items-center justify-between'>
        <div className='space-y-1'>
          <h1 className='text-2xl font-semibold'>{t('index.title')}</h1>
          <p className='text-muted-foreground text-sm'>
            {t('index.description')}
          </p>
        </div>

        <Button asChild>
          <Link to={getRoutePath(ROUTES.CREATE_AUDIT)}>
            <PlusIcon className='size-4' />
            {t('index.createAudit')}
          </Link>
        </Button>
      </div>

      <div className='grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3'>
        {audits.map(audit => (
          <AuditCard key={audit.id} audit={audit} />
        ))}
      </div>
    </div>
  )
}

export default AuditScreen
